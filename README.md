# Voz Local

**Voz a texto 100% local en tu Mac. Sin nube, sin suscripción, sin internet.**

Presiona el atajo de teclado, habla, y el texto aparece donde está tu cursor — en cualquier app.

[![Release](https://img.shields.io/github/v/release/emeforero/voz-local?style=flat-square)](https://github.com/emeforero/voz-local/releases/latest)
[![macOS](https://img.shields.io/badge/macOS-12%2B-black?style=flat-square&logo=apple)](https://github.com/emeforero/voz-local/releases/latest)
[![Apple Silicon](https://img.shields.io/badge/Apple_Silicon-M1%2FM2%2FM3%2FM4-black?style=flat-square)](https://github.com/emeforero/voz-local/releases/latest)
[![License: MIT](https://img.shields.io/badge/License-MIT-black?style=flat-square)](LICENSE)

---

## Descarga

**[⬇ Descargar última versión para macOS](https://github.com/emeforero/voz-local/releases/latest)**

> Requiere macOS 12 (Monterey) o superior · Apple Silicon (M1/M2/M3/M4)

---

## Qué hace

- **Transcripción local** con [Whisper](https://github.com/openai/whisper) (ggml) acelerado por Metal — el modelo corre completamente en tu Mac, nada sale a internet
- **Atajo global** configurable (por defecto `Alt+Space`) — funciona aunque la ventana esté cerrada
- **Push to talk** o modo toggle
- **Pegado automático** donde tengas el cursor
- **Widget flotante** con visualización de audio en tiempo real
- **Historial** de transcripciones con reproducción de audio
- **Vocabulario personalizado** para mejorar el reconocimiento de términos técnicos
- **Inicio automático** con el sistema

---

## Instalación

1. Descarga el `.dmg` desde [Releases](https://github.com/emeforero/voz-local/releases/latest)
2. Abre el `.dmg` y arrastra **Voz Local** a Aplicaciones
3. Abre la app — aparece en la barra de menú (no en el Dock)
4. Concede los permisos de **Micrófono** y **Accesibilidad**
5. Descarga el modelo Whisper desde la pantalla de bienvenida
6. Presiona `Alt+Space` para dictar

> **Si macOS dice "dañado":** abre Terminal y ejecuta `sudo xattr -cr /Applications/Voz\ Local.app`

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

Los modelos no se incluyen en el instalador. Al abrir la app por primera vez, elige y descarga el modelo directamente desde la pantalla de bienvenida. Se guardan en `~/Library/Application Support/com.vozlocal.app/models/` y funcionan sin conexión una vez descargados.

---

## Construir desde código fuente

### Requisitos

- [Rust](https://rustup.rs) (stable)
- [Node.js](https://nodejs.org) 18+
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

| Capa | Tecnología |
|---|---|
| Frontend | SvelteKit + TypeScript (adapter-static) |
| Backend | Rust + Tauri v2 |
| Transcripción | [whisper-rs](https://github.com/tazz4843/whisper-rs) + Metal (GPU Apple Silicon) |
| Audio | cpal |
| Corrección de texto | [strsim](https://github.com/dguo/strsim-rs) (Jaro-Winkler) |
| Widget | NSVisualEffectView (vibrancy nativa macOS) |

---

## Changelog

### v1.5.0
- **Transcripción más rápida**: detección de silencio por energía (VAD) antes de enviar audio a Whisper — reduce el audio procesado entre 20-70% en grabaciones típicas
- **Vocabulario personalizado**: lista de palabras/frases que se pasan a Whisper como `initial_prompt` para mejorar el reconocimiento de términos técnicos (GitHub, Claude Code, Node.js, TypeScript, etc.)
- **Corrección post-transcripción**: comparación fuzzy con Jaro-Winkler entre el texto transcripto y el vocabulario personalizado — corrige errores leves como "tyepscript" → "TypeScript"
- **Umbral de corrección configurable**: slider en Ajustes para controlar qué tan agresiva es la corrección (0.85 por defecto)

### v0.1.0
- Release inicial: transcripción local con Whisper, atajo global, push-to-talk, widget flotante, historial, pegado automático

---

## Licencia

MIT
