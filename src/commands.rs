use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommandType {
    AI,
    TextReplacer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    pub trigger: String,
    pub prompt: Option<String>,
    pub replacement: Option<String>,
    pub cmd_type: CommandType,
}

#[derive(Debug, Default)]
pub struct CommandManager {
    builtins: Vec<Command>,
}

impl CommandManager {
    pub fn default() -> Self {
        let builtins = vec![
            Command { trigger: "?fix".into(), prompt: Some("Fix grammar and punctuation".into()), replacement: None, cmd_type: CommandType::AI },
            Command { trigger: "?formal".into(), prompt: Some("Rewrite in a formal tone".into()), replacement: None, cmd_type: CommandType::AI },
            Command { trigger: "?reply".into(), prompt: Some("Generate a contextual reply".into()), replacement: None, cmd_type: CommandType::AI },
            Command { trigger: "?summarize".into(), prompt: Some("Summarize the text concisely".into()), replacement: None, cmd_type: CommandType::AI },
        ];
        CommandManager { builtins }
    }

    // Longest-match suffix search. Returns (Command, stripped_text)
    pub fn find_command(&self, input: &str) -> Option<(Command, String)> {
        let trimmed = input.trim_end();
        // Iterate builtins and find the longest trigger that matches suffix
        let mut matches: Vec<&Command> = self.builtins.iter()
            .filter(|c| trimmed.ends_with(&c.trigger))
            .collect();
        if matches.is_empty() {
            return None;
        }
        // pick longest trigger
        matches.sort_by_key(|c| c.trigger.len());
        let cmd = matches.last().unwrap().clone();
        // strip trigger from end
        let stripped = trimmed[..trimmed.len() - cmd.trigger.len()].trim_end().to_string();
        Some((cmd.clone(), stripped))
    }
}
