use crate::commands::Command;
use anyhow::{anyhow, Result};
use reqwest::blocking::Client;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde_json::{json, Value};
use std::env;

#[derive(Debug, Default)]
pub struct DummyProvider {}

impl DummyProvider {
    pub fn transform(&self, cmd: &Command, text: &str) -> Result<String> {
        match cmd.cmd_type {
            crate::commands::CommandType::AI => {
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

#[derive(Debug)]
pub struct OpenAIProvider {
    client: Client,
    endpoint: String,
    api_key: Option<String>,
    model: String,
}

impl OpenAIProvider {
    pub fn new(endpoint: Option<String>, api_key: Option<String>, model: Option<String>) -> Self {
        let client = Client::builder().build().expect("failed to build reqwest client");
        OpenAIProvider {
            client,
            endpoint: endpoint.unwrap_or_else(|| "https://api.openai.com/v1/chat/completions".into()),
            api_key,
            model: model.unwrap_or_else(|| "gpt-3.5-turbo".into()),
        }
    }

    pub fn transform(&self, cmd: &Command, text: &str) -> Result<String> {
        // If no API key is configured, fallback to DummyProvider behavior
        if self.api_key.is_none() {
            let d = DummyProvider::default();
            return d.transform(cmd, text);
        }

        match cmd.cmd_type {
            crate::commands::CommandType::TextReplacer => {
                if let Some(repl) = &cmd.replacement {
                    return Ok(repl.clone());
                } else {
                    return Ok(text.to_string());
                }
            }
            crate::commands::CommandType::AI => {
                let system_prompt = cmd.prompt.clone().unwrap_or_else(|| "You are a helpful assistant that returns only the transformed text.".into());

                let body = json!({
                    "model": self.model,
                    "messages": [
                        { "role": "system", "content": system_prompt },
                        { "role": "user", "content": text }
                    ],
                    "max_tokens": 1024,
                    "temperature": 0.2
                });

                let key = self.api_key.as_ref().unwrap();
                let res = self.client.post(&self.endpoint)
                    .header(AUTHORIZATION, format!("Bearer {}", key))
                    .header(CONTENT_TYPE, "application/json")
                    .json(&body)
                    .send()
                    .map_err(|e| anyhow!("request error: {}", e))?;

                if !res.status().is_success() {
                    return Err(anyhow!("provider returned error status: {}", res.status()));
                }

                let v: Value = res.json().map_err(|e| anyhow!("invalid json: {}", e))?;

                // Try to extract OpenAI-style response: choices[0].message.content
                if let Some(content) = v.get("choices")
                    .and_then(|c| c.get(0))
                    .and_then(|ch| ch.get("message"))
                    .and_then(|m| m.get("content"))
                    .and_then(|ct| ct.as_str())
                {
                    return Ok(content.trim().to_string());
                }

                // Fallback: try top-level text
                if let Some(text_out) = v.get("text").and_then(|t| t.as_str()) {
                    return Ok(text_out.trim().to_string());
                }

                Err(anyhow!("unexpected provider response: {}", v))
            }
        }
    }

    pub fn from_env() -> Self {
        let api_key = env::var("OPENAI_API_KEY").ok();
        let endpoint = env::var("OPENAI_API_ENDPOINT").ok();
        let model = env::var("OPENAI_API_MODEL").ok();
        OpenAIProvider::new(endpoint, api_key, model)
    }
}
