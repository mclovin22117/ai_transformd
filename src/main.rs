mod commands;
mod providers;
mod config;
mod input;
mod replace;

use clap::Parser;
use commands::CommandManager;
use providers::OpenAIProvider;
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
        // Instantiate provider from environment (OPENAI_API_KEY, OPENAI_API_ENDPOINT, OPENAI_API_MODEL)
        let provider = OpenAIProvider::from_env();
        let out = provider.transform(&cmd, &stripped)?;
        println!("Result:\n{}", out);
    } else {
        println!("No command detected in input.");
    }

    Ok(())
}
