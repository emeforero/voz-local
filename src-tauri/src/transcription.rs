use std::sync::{
    atomic::{AtomicBool, AtomicU64, Ordering},
    Arc, Mutex,
};
use anyhow::{anyhow, Result};
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

// Cache the loaded model so we don't re-read 469 MB from disk on every recording.
static MODEL_CACHE: Mutex<Option<(String, WhisperContext)>> = Mutex::new(None);

pub struct AudioCapture {
    pub sample_rate: u32,
    samples: Arc<Mutex<Vec<f32>>>,
    stop: Arc<AtomicBool>,
    thread: Option<std::thread::JoinHandle<()>>,
}

impl AudioCapture {
    pub fn start(on_level: impl Fn(f32) + Send + 'static) -> Result<Self> {
        use cpal::traits::{DeviceTrait, HostTrait};

        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .ok_or_else(|| anyhow!("No se encontró micrófono"))?;

        let config = device.default_input_config()?;
        let sample_rate = config.sample_rate().0;
        let channels = config.channels() as usize;

        let samples = Arc::new(Mutex::new(Vec::<f32>::new()));
        let stop = Arc::new(AtomicBool::new(false));
        let last_emit_ms = Arc::new(AtomicU64::new(0));

        let samples_thread = Arc::clone(&samples);
        let stop_thread = Arc::clone(&stop);

        let thread = std::thread::spawn(move || {
            use cpal::traits::StreamTrait;
            let samples_cb = Arc::clone(&samples_thread);
            let last_emit = Arc::clone(&last_emit_ms);

            let stream = device
                .build_input_stream(
                    &config.into(),
                    move |data: &[f32], _| {
                        {
                            let mut buf = samples_cb.lock().unwrap();
                            for frame in data.chunks(channels) {
                                let mono = frame.iter().sum::<f32>() / channels as f32;
                                buf.push(mono);
                            }
                        }
                        let now_ms = std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_millis() as u64;
                        let prev = last_emit.load(Ordering::Relaxed);
                        if now_ms.saturating_sub(prev) >= 50 {
                            last_emit.store(now_ms, Ordering::Relaxed);
                            let rms = if data.is_empty() {
                                0.0_f32
                            } else {
                                (data.iter().map(|s| s * s).sum::<f32>() / data.len() as f32).sqrt()
                            };
                            on_level(rms);
                        }
                    },
                    |err| eprintln!("cpal error: {err}"),
                    None,
                )
                .expect("failed to build input stream");

            stream.play().expect("failed to start stream");
            while !stop_thread.load(Ordering::Relaxed) {
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
        });

        Ok(Self { sample_rate, samples, stop, thread: Some(thread) })
    }

    pub fn stop(mut self) -> (Vec<f32>, u32) {
        self.stop.store(true, Ordering::SeqCst);
        if let Some(t) = self.thread.take() { let _ = t.join(); }
        let samples = self.samples.lock().unwrap().clone();
        (samples, self.sample_rate)
    }
}

pub fn transcribe(model_path: &str, samples: &[f32], sample_rate: u32, language: &str) -> Result<String> {
    let mut cache = MODEL_CACHE.lock().unwrap();

    // Load or reload model only when path changes
    let needs_load = cache.as_ref().map(|(p, _)| p.as_str() != model_path).unwrap_or(true);
    if needs_load {
        eprintln!("[voz-local] loading model: {model_path}");
        let mut ctx_params = WhisperContextParameters::default();
        ctx_params.use_gpu(true);  // force Metal GPU on Apple Silicon
        let ctx = WhisperContext::new_with_params(model_path, ctx_params)?;
        *cache = Some((model_path.to_string(), ctx));
    }

    let ctx = &cache.as_ref().unwrap().1;
    let mut state = ctx.create_state()?;

    let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
    params.set_n_threads(4);           // 4 performance cores on Apple Silicon
    params.set_n_max_text_ctx(224);    // reduce token context → faster sampling
    params.set_print_progress(false);
    params.set_print_realtime(false);
    params.set_print_timestamps(false);
    params.set_suppress_blank(true);
    params.set_no_speech_thold(0.6);

    let lang = if language == "auto" { None } else { Some(language) };
    params.set_language(lang);

    let audio = resample(samples, sample_rate as usize, 16000);
    state.full(params, &audio)?;

    let n = state.full_n_segments()?;
    let text = (0..n)
        .filter_map(|i| state.full_get_segment_text(i).ok())
        .collect::<Vec<_>>()
        .join(" ")
        .trim()
        .to_string();

    Ok(text)
}

fn resample(samples: &[f32], from_hz: usize, to_hz: usize) -> Vec<f32> {
    if from_hz == to_hz || samples.is_empty() { return samples.to_vec(); }
    let ratio = from_hz as f64 / to_hz as f64;
    let out_len = (samples.len() as f64 / ratio) as usize;
    (0..out_len).map(|i| {
        let pos = i as f64 * ratio;
        let lo = pos as usize;
        let hi = (lo + 1).min(samples.len() - 1);
        let frac = (pos - lo as f64) as f32;
        samples[lo] * (1.0 - frac) + samples[hi] * frac
    }).collect()
}
