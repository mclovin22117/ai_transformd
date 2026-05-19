# AI Transform Daemon (MVP)

Minimal scaffold for a Linux-first system-wide text transformation daemon inspired by SwiftSlate.

This repo provides a small Rust CLI that simulates trigger detection and provider flow.

Run (requires Rust toolchain):

```bash
cargo run -- --text "i dont no whats hapening ?fix"
```

What this scaffold contains:
- `src/commands.rs`: command model and longest-suffix matching
- `src/providers.rs`: `DummyProvider` that returns a simple transformed string
- `src/config.rs`: basic config loader (TOML)
- `src/input.rs` and `src/replace.rs`: stubs for future X11/Wayland implementations

Next steps:
- Implement real provider clients (OpenAI-compatible, Gemini)
- Add global key listener and clipboard monitoring for X11
- Implement replacement strategies using XTest / clipboard fallback
- Add tray UI and settings (Tauri or GTK)
