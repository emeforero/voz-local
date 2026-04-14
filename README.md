# Voz Local

**100% local speech-to-text for your Mac. No cloud, no subscription, no internet.**

Press a keyboard shortcut, speak, and the text appears wherever your cursor is — in any app.

[![Release](https://img.shields.io/github/v/release/emeforero/voz-local?style=flat-square)](https://github.com/emeforero/voz-local/releases/latest)
[![macOS](https://img.shields.io/badge/macOS-12%2B-black?style=flat-square&logo=apple)](https://github.com/emeforero/voz-local/releases/latest)
[![Apple Silicon](https://img.shields.io/badge/Apple_Silicon-M1%2FM2%2FM3%2FM4-black?style=flat-square)](https://github.com/emeforero/voz-local/releases/latest)
[![License: MIT](https://img.shields.io/badge/License-MIT-black?style=flat-square)](LICENSE)

---

## Download

**[⬇ Download Voz Local for macOS](https://github.com/emeforero/voz-local/releases/latest)**

> Requires macOS 12 (Monterey) or later · Apple Silicon (M1/M2/M3/M4)

---

## Features

- **Local transcription** using [Whisper](https://github.com/openai/whisper) (ggml) accelerated by Metal — the model runs entirely on your Mac, nothing leaves your device
- **Global shortcut** (default `Alt+Space`) — works even when the window is closed
- **Push-to-talk** or toggle mode
- **Auto-paste** wherever your cursor is
- **Floating widget** with real-time audio visualization
- **Transcription history** with audio playback
- **Custom vocabulary** to improve recognition of technical terms
- **Launch at login** support

---

## Installation

1. Download the `.dmg` from [Releases](https://github.com/emeforero/voz-local/releases/latest)
2. Open the `.dmg` and drag **Voz Local** to Applications
3. Open the app — it lives in the menu bar (not the Dock)
4. Grant **Microphone** and **Accessibility** permissions when prompted
5. Download a Whisper model from the welcome screen
6. Press `Alt+Space` to start dictating

> **If macOS says the app is damaged:** open Terminal and run `sudo xattr -cr /Applications/Voz\ Local.app`

---

## Permissions

| Permission | Why |
|---|---|
| **Microphone** | Capture audio input |
| **Accessibility** | Auto-paste transcribed text at your cursor position |

> Without Accessibility permission the text is still copied to the clipboard — you can paste it manually with `Cmd+V`.

---

## Models

| Model | Size | Speed | Accuracy |
|---|---|---|---|
| Whisper Large v3 Turbo | 809 MB | ★★★★☆ | ★★★★★ |
| Whisper Base | 141 MB | ★★★★★ | ★★★☆☆ |

Models are not bundled with the installer. On first launch, choose and download your preferred model directly from the welcome screen. Models are stored in `~/Library/Application Support/com.vozlocal.app/models/` and work offline once downloaded.

---

## Building from source

### Requirements

- [Rust](https://rustup.rs) (stable)
- [Node.js](https://nodejs.org) 18+
- macOS 12+

```bash
git clone https://github.com/emeforero/voz-local
cd voz-local
npm install
npm run tauri dev        # development
npm run tauri build      # production
```

---

## Tech stack

| Layer | Technology |
|---|---|
| Frontend | SvelteKit + TypeScript (adapter-static) |
| Backend | Rust + Tauri v2 |
| Transcription | [whisper-rs](https://github.com/tazz4843/whisper-rs) + Metal (Apple Silicon GPU) |
| Audio capture | cpal |
| Word correction | [strsim](https://github.com/dguo/strsim-rs) (Jaro-Winkler) |
| Widget | NSVisualEffectView (native macOS vibrancy) |

---

## Changelog

### v1.5.0
- **Faster transcription**: energy-based VAD trims leading/trailing silence before Whisper inference, reducing processed audio by 20–70% on typical recordings
- **Custom vocabulary**: user-defined word list passed to Whisper as `initial_prompt` to bias recognition toward technical terms (GitHub, Claude Code, Node.js, TypeScript, etc.)
- **Post-transcription word correction**: Jaro-Winkler fuzzy matching between the transcript and the custom vocabulary — corrects typos like "tyepscript" → "TypeScript"
- **Configurable correction threshold**: slider in Settings to control how aggressively corrections are applied (default 0.85)

### v0.1.0
- Initial release: local Whisper transcription, global shortcut, push-to-talk, floating widget, transcription history, auto-paste

---

## License

MIT
