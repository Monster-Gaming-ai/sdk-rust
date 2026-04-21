// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Luxedeum, LLC d/b/a Monster Gaming

//! # Monster Gaming SDK for Rust
//!
//! Official Rust client for [Monster Gaming](https://monstergaming.ai) —
//! an AI-powered game development platform for Unreal Engine, Unity, Godot,
//! and bespoke engines.
//!
//! ## Quick Start
//!
//! ```no_run
//! use monstergaming::{MonsterGaming, ChatMessage};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), monstergaming::Error> {
//!     let client = MonsterGaming::new("mg_your_api_key");
//!
//!     let response = client
//!         .chat_completion("monster-gpt", vec![
//!             ChatMessage::user("Generate a UE5 C++ character controller with double jump"),
//!         ])
//!         .await?;
//!
//!     println!("{}", response.choices[0].message.content);
//!     Ok(())
//! }
//! ```

use reqwest::Client;
use serde::{Deserialize, Serialize};

/// Error type for Monster Gaming API operations.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Monster Gaming API error: {status} — {message}")]
    Api {
        status: u16,
        message: String,
        body: Option<serde_json::Value>,
    },

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

/// A chat message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

impl ChatMessage {
    pub fn system(content: impl Into<String>) -> Self {
        Self { role: "system".into(), content: content.into() }
    }

    pub fn user(content: impl Into<String>) -> Self {
        Self { role: "user".into(), content: content.into() }
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        Self { role: "assistant".into(), content: content.into() }
    }
}

/// Chat completion request body.
#[derive(Debug, Serialize)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,
}

/// A single completion choice.
#[derive(Debug, Deserialize)]
pub struct Choice {
    pub index: u32,
    pub message: ChatMessage,
    pub finish_reason: String,
}

/// Token usage information.
#[derive(Debug, Deserialize)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

/// Chat completion response.
#[derive(Debug, Deserialize)]
pub struct ChatCompletionResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Usage,
}

/// A model descriptor.
#[derive(Debug, Deserialize)]
pub struct Model {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub owned_by: String,
}

/// Model list response.
#[derive(Debug, Deserialize)]
pub struct ModelList {
    pub object: String,
    pub data: Vec<Model>,
}

/// Monster Gaming API client.
pub struct MonsterGaming {
    api_key: String,
    base_url: String,
    client: Client,
}

impl MonsterGaming {
    /// Create a new client with the given API key.
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: "https://api.monstergaming.ai".into(),
            client: Client::new(),
        }
    }

    /// Set a custom base URL (for testing or self-hosted deployments).
    pub fn with_base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into().trim_end_matches('/').to_string();
        self
    }

    /// Create a chat completion.
    pub async fn chat_completion(
        &self,
        model: impl Into<String>,
        messages: Vec<ChatMessage>,
    ) -> Result<ChatCompletionResponse, Error> {
        let req = ChatCompletionRequest {
            model: model.into(),
            messages,
            temperature: None,
            max_tokens: None,
            top_p: None,
            stop: None,
        };
        self.chat_completion_full(req).await
    }

    /// Create a chat completion with full request control.
    pub async fn chat_completion_full(
        &self,
        request: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse, Error> {
        let resp = self
            .client
            .post(format!("{}/v1/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body: Option<serde_json::Value> = resp.json().await.ok();
            return Err(Error::Api {
                status,
                message: body
                    .as_ref()
                    .and_then(|b| b["error"]["message"].as_str())
                    .unwrap_or("Unknown error")
                    .to_string(),
                body,
            });
        }

        Ok(resp.json().await?)
    }

    /// List available models.
    pub async fn list_models(&self) -> Result<ModelList, Error> {
        let resp = self
            .client
            .get(format!("{}/v1/models", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body: Option<serde_json::Value> = resp.json().await.ok();
            return Err(Error::Api {
                status,
                message: "Failed to list models".into(),
                body,
            });
        }

        Ok(resp.json().await?)
    }
}