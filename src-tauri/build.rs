fn main() {
    tauri_build::build();
    // Note: codesign must run AFTER the linker finishes, so it cannot run here.
    // Use scripts/sign-dev.sh after each `cargo build` / `npm run tauri dev`.
}
