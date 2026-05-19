use crate::commands::Command;
use anyhow::Result;

#[derive(Debug, Default)]
pub struct DummyProvider {}

impl DummyProvider {
    pub fn transform(&self, cmd: &Command, text: &str) -> Result<String> {
        match cmd.cmd_type {
            crate::commands::CommandType::AI => {
                // For MVP return a deterministic transformed string
                let prompt = cmd.prompt.clone().unwrap_or_else(|| "Transform".into());
                Ok(format!("[{}] {} -> {}", cmd.trigger, prompt, text))
            }
            crate::commands::CommandType::TextReplacer => {
                if let Some(repl) = &cmd.replacement {
                    Ok(repl.clone())
                } else {
                    Ok(text.to_string())
                }
            }
        }
    }
}
