mod commands;
mod providers;
mod config;
mod input;
mod replace;

use clap::Parser;
use commands::{CommandManager, Command};
use providers::DummyProvider;
use tracing_subscriber;

#[derive(Parser)]
struct Cli {
    /// Text to process (include trigger at end)
    #[arg(short, long)]
    text: String,
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();

    let manager = CommandManager::default();
    if let Some((cmd, stripped)) = manager.find_command(&cli.text) {
        println!("Detected command: {}", cmd.trigger);
        // For MVP use a local dummy provider
        let provider = DummyProvider::default();
        let out = provider.transform(cmd, &stripped)?;
        println!("Result:\n{}", out);
    } else {
        println!("No command detected in input.");
    }

    Ok(())
}
