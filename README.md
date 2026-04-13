# Voz Local

**Voz a texto 100% local en tu Mac. Sin nube, sin suscripción, sin internet.**

Presiona el atajo de teclado, habla, y el texto aparece donde está tu cursor — en cualquier app.

---

## Descarga

**[⬇ Descargar Voz Local para macOS](../../releases/latest)**

> Requiere macOS 12 (Monterey) o superior · Apple Silicon (M1/M2/M3/M4)

---

## Qué hace

- **Transcripción local** con [Whisper](https://github.com/openai/whisper) (ggml) — el modelo corre en tu Mac, nada sale a internet
- **Atajo global** configurable (por defecto `Alt+Space`) — funciona aunque la ventana esté cerrada
- **Push to talk** o modo toggle
- **Pegado automático** donde tengas el cursor
- **Widget flotante** con visualización de audio en tiempo real
- **Historial** de transcripciones con reproducción de audio
- **Inicio automático** con el sistema

---

## Instalación

1. Descarga el `.dmg` desde [Releases](../../releases/latest)
2. Abre el `.dmg` y arrastra **Voz Local** a Aplicaciones
3. Abre la app — aparece en la barra de menú (no en el Dock)
4. En la pantalla de bienvenida, concede los permisos de **Micrófono** y **Accesibilidad**
5. ¡Listo! Presiona `Alt+Space` para dictar

---

## Permisos requeridos

| Permiso | Para qué |
|---|---|
| **Micrófono** | Capturar audio |
| **Accesibilidad** | Pegar el texto automáticamente donde escribes |

> Sin Accesibilidad el texto igual se copia al portapapeles — puedes pegarlo manualmente con `Cmd+V`.

---

## Modelos disponibles

| Modelo | Tamaño | Velocidad | Precisión |
|---|---|---|---|
| Whisper Large v3 Turbo | 809 MB | ★★★★☆ | ★★★★★ |
| Whisper Base | 141 MB | ★★★★★ | ★★★☆☆ |

Los modelos se incluyen en el bundle de la app.

---

## Construir desde código fuente

### Requisitos
- [Rust](https://rustup.rs) (stable)
- [Node.js](https://nodejs.org) 18+
- [Tauri CLI](https://tauri.app/start/prerequisites/)
- macOS 12+

```bash
git clone https://github.com/emeforero/voz-local
cd voz-local
npm install
npm run tauri dev        # desarrollo
npm run tauri build      # producción
```

---

## Stack técnico

- **Frontend**: SvelteKit + TypeScript (adapter-static)
- **Backend**: Rust + Tauri v2
- **Transcripción**: [whisper-rs](https://github.com/tazz4843/whisper-rs) con aceleración Metal (GPU Apple Silicon)
- **Audio**: cpal
- **Ventana widget**: transparencia nativa macOS (macOSPrivateApi) + NSVisualEffectView (vibrancy)

---

## Licencia

MIT
