mod commands;
mod providers;
mod config;
mod input;
mod replace;

use clap::Parser;
use commands::CommandManager;
use providers::OpenAIProvider;
use tracing_subscriber;
use std::time::Duration;
use arboard::Clipboard;
use input::{start_hotkey_listener, start_clipboard_watcher};

#[derive(Parser)]
struct Cli {
    /// Text to process (include trigger at end)
    #[arg(short, long)]
    text: String,
    /// Run in hotkey listener mode (capture clipboard on hotkey)
    #[arg(long)]
    hotkey: bool,
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();

    let manager = CommandManager::default();
    let provider = OpenAIProvider::from_env();

    if cli.hotkey {
        println!("Hotkey mode: press Ctrl+Shift+Space to capture clipboard and process commands.");
        let receiver = start_hotkey_listener();
        // also start a clipboard watcher to pick up changes (optional)
        let _clip_watcher = start_clipboard_watcher(Duration::from_millis(500));

        for snap in receiver.iter() {
            println!("Captured clipboard text: {}", snap.text);
            if let Some((cmd, stripped)) = manager.find_command(&snap.text) {
                println!("Detected command: {}", cmd.trigger);
                match provider.transform(&cmd, &stripped) {
                    Ok(out) => {
                        println!("Transformed result:\n{}", out);
                        // Replace clipboard contents with transformed text so user can paste
                        if let Ok(mut cb) = Clipboard::new() {
                            let _ = cb.set_text(out.clone());
                        }
                    }
                    Err(e) => {
                        eprintln!("provider error: {}", e);
                    }
                }
            } else {
                println!("No command detected in clipboard text.");
            }
        }
    } else {
        if let Some((cmd, stripped)) = manager.find_command(&cli.text) {
            println!("Detected command: {}", cmd.trigger);
            let out = provider.transform(&cmd, &stripped)?;
            println!("Result:\n{}", out);
        } else {
            println!("No command detected in input.");
        }
    }

    Ok(())
}
