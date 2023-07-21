use super::{ChatModel, CompletionModel};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use versa_common::traits::Config;

//-------------------------------------------------------------------------------------------------
// Constants
//-------------------------------------------------------------------------------------------------

pub const OPENAI_COMPLETION_URL: &str = "https://api.openai.com/v1/completions";
pub const OPENAI_CHAT_URL: &str = "https://api.openai.com/v1/chat/completions";

//-------------------------------------------------------------------------------------------------
// Types
//-------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Attributes {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub echo: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub best_of: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<HashMap<u64, i8>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatConfig {
    pub model: ChatModel,

    #[serde(flatten)]
    pub attributes: Attributes,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CompletionConfig {
    pub model: CompletionModel,

    #[serde(flatten)]
    pub attributes: Attributes,
}

//-------------------------------------------------------------------------------------------------
// Traits
//-------------------------------------------------------------------------------------------------

pub trait OpenAIConfig: Config {
    fn get_url(&self) -> &str;
}

//-------------------------------------------------------------------------------------------------
// Trait Implementations
//-------------------------------------------------------------------------------------------------

impl Config for ChatConfig {}

impl Config for CompletionConfig {}

impl OpenAIConfig for ChatConfig {
    fn get_url(&self) -> &str {
        OPENAI_CHAT_URL
    }
}

impl OpenAIConfig for CompletionConfig {
    fn get_url(&self) -> &str {
        OPENAI_COMPLETION_URL
    }
}

impl Default for ChatConfig {
    fn default() -> Self {
        Self {
            model: ChatModel::GPT3_5Turbo,
            attributes: Default::default(),
        }
    }
}

impl Default for CompletionConfig {
    fn default() -> Self {
        Self {
            model: CompletionModel::TextDaVinci003,
            attributes: Attributes::default(),
        }
    }
}
