//! This module contains implementations of OpenAI models.

use super::{
    ChatConfig, ChatModel, ChatModelStream, CompletionConfig, CompletionModel,
    CompletionModelStream, ModelType, OpenAIConfig,
};
use crate::{
    openai::{APIError, OpenAIError},
    traits::{Model, Output},
    ModelError,
};
use async_trait::async_trait;
use derivative::Derivative;
use reqwest::{header::AUTHORIZATION, Client};
use reqwest_eventsource::RequestBuilderExt;
use serde::{Deserialize, Serialize};
use std::{env, fmt::Debug};
use strum_macros::Display;

//-------------------------------------------------------------------------------------------------
// Aliases
//-------------------------------------------------------------------------------------------------

pub type OpenAICompletionModel = OpenAI<CompletionModel>;
pub type OpenAIChatModel = OpenAI<ChatModel>;
pub type OpenAIModel = OpenAIChatModel;

pub type CompletionModelResponse = ModelResponse<CompletionChoice>;
pub type ChatModelResponse = ModelResponse<ChatChoice>;

//-------------------------------------------------------------------------------------------------
// Types
//-------------------------------------------------------------------------------------------------

/// An OpenAI language model.
#[derive(Derivative, Debug)]
pub struct OpenAI<M>
where
    M: ModelType,
{
    /// The configuration of the model.
    pub config: M::Config,

    // OpenAI API key.
    #[derivative(Debug = "ignore")]
    api_key: String,
}

#[derive(Debug, Deserialize)]
pub struct ChatChoice {
    pub index: u64,
    pub message: ChatMessage,
    pub finish_reason: String,
}

#[derive(Debug, Deserialize)]
pub struct CompletionChoice {
    pub index: u64,
    pub text: String,
    pub logprobs: Option<u8>,
    pub finish_reason: String,
}

#[derive(Debug, Serialize, Deserialize, Display)]
#[serde(rename_all = "snake_case")]
pub enum ChatRole {
    #[strum(serialize = "system")]
    System,

    #[strum(serialize = "user")]
    User,

    #[strum(serialize = "assistant")]
    Assistant,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: ChatRole,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct ModelResponse<T> {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<T>,
}

#[derive(Debug, Serialize, Default)]
pub struct ChatBody {
    pub messages: Vec<ChatMessage>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    #[serde(flatten)]
    pub config: ChatConfig,
}

#[derive(Debug, Serialize, Default)]
pub struct CompletionBody {
    pub prompt: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    #[serde(flatten)]
    pub config: CompletionConfig,
}

//-------------------------------------------------------------------------------------------------
// Methods
//-------------------------------------------------------------------------------------------------

impl<M> OpenAI<M>
where
    M: ModelType,
{
    /// Creates a new OpenAI language model ne it takes an API key that can be used to make request to associated endpoint.
    ///
    /// OpenAI API keys can obtained at https://platform.openai.com/account/api-keys.
    pub fn new(config: M::Config, api_key: impl Into<String>) -> Self {
        Self {
            config,
            api_key: api_key.into(),
        }
    }

    /// Creates a new OpenAI language model with the given configuration.
    /// This function expects to find the API key in `OPENAI_API_KEY` environment variable.
    ///
    /// OpenAI API keys can obtained at https://platform.openai.com/account/api-keys.
    pub fn with_config(config: M::Config) -> Self {
        Self::new(config, env::var("OPENAI_API_KEY").unwrap())
    }
}

impl OpenAIChatModel {
    // TODO(nyprothegeek): Support stream.
    /// Sends a request to the OpenAI API to get a completion.
    pub async fn get_completion(
        &self,
        _messages: Vec<ChatMessage>,
    ) -> Result<ChatModelResponse, OpenAIError> {
        todo!()
    }
}

impl OpenAICompletionModel {
    // TODO(nyprothegeek): Support stream.
    /// Sends a request to the OpenAI API to get a completion.
    pub async fn get_completion(
        &self,
        _prompt: impl Into<String>,
    ) -> Result<CompletionModelResponse, OpenAIError> {
        todo!()
    }
}

//-------------------------------------------------------------------------------------------------
// Trait Implementations
//-------------------------------------------------------------------------------------------------

#[async_trait(?Send)]
impl<M> Model for OpenAI<M>
where
    M: ModelType,
{
    type Config = M::Config;

    async fn prompt<O>(&self, prompt: impl Into<String>) -> Result<O, ModelError>
    where
        O: Output<Self>,
    {
        O::from_call(prompt, self).await
    }

    async fn prompt_with_config<O>(
        &self,
        prompt: String,
        config: Self::Config,
    ) -> Result<O, ModelError>
    where
        O: Output<Self>,
    {
        O::from_call_with_config(prompt, self, config).await
    }

    fn get_config(&self) -> &Self::Config {
        &self.config
    }
}

#[async_trait(?Send)]
impl Output<OpenAIChatModel> for String {
    async fn from_call_with_config(
        prompt: impl Into<String>,
        model: &OpenAIChatModel,
        config: ChatConfig,
    ) -> Result<Self, ModelError> {
        let request = Client::new()
            .post(model.config.get_url())
            .header(AUTHORIZATION, format!("Bearer {}", model.api_key))
            .json(&ChatBody {
                messages: vec![ChatMessage {
                    content: prompt.into(),
                    role: ChatRole::User,
                }],
                config,
                ..Default::default()
            });

        let response = request.send().await.map_err(OpenAIError::Reqwest)?;

        if !response.status().is_success() {
            let error: APIError = response.json().await.map_err(OpenAIError::Reqwest)?;
            return Err(OpenAIError::API(error).into());
        }

        let response: ChatModelResponse = response.json().await.map_err(OpenAIError::Reqwest)?;

        #[cfg(feature = "log")]
        log::debug!("response: {response:#?}");

        Ok(response
            .choices
            .get(0)
            .ok_or(OpenAIError::CompletionMissing)?
            .message
            .content
            .clone())
    }
}

#[async_trait(?Send)]
impl Output<OpenAICompletionModel> for String {
    async fn from_call_with_config(
        prompt: impl Into<String>,
        model: &OpenAICompletionModel,
        config: CompletionConfig,
    ) -> Result<Self, ModelError> {
        let request = Client::new()
            .post(model.config.get_url())
            .header(AUTHORIZATION, format!("Bearer {}", model.api_key))
            .json(&CompletionBody {
                prompt: prompt.into(),
                config,
                ..Default::default()
            });

        let response = request.send().await.map_err(OpenAIError::Reqwest)?;

        if !response.status().is_success() {
            let error: APIError = response.json().await.map_err(OpenAIError::Reqwest)?;
            return Err(OpenAIError::API(error).into());
        }

        let response: CompletionModelResponse =
            response.json().await.map_err(OpenAIError::Reqwest)?;

        #[cfg(feature = "log")]
        log::debug!("response: {response:#?}");

        Ok(response
            .choices
            .get(0)
            .ok_or(OpenAIError::CompletionMissing)?
            .text
            .clone())
    }
}

#[async_trait(?Send)]
impl Output<OpenAIChatModel> for ChatModelStream {
    async fn from_call_with_config(
        prompt: impl Into<String>,
        model: &OpenAIChatModel,
        config: ChatConfig,
    ) -> Result<Self, ModelError> {
        let request = Client::new()
            .post(model.config.get_url())
            .header(AUTHORIZATION, format!("Bearer {}", model.api_key))
            .json(&ChatBody {
                messages: vec![ChatMessage {
                    content: prompt.into(),
                    role: ChatRole::User,
                }],
                stream: Some(true),
                config,
            });

        let event_src = request
            .eventsource()
            .map_err(|_| OpenAIError::CannotCloneRequestError)?;

        Ok(ChatModelStream::new(event_src))
    }
}

#[async_trait(?Send)]
impl Output<OpenAICompletionModel> for CompletionModelStream {
    async fn from_call_with_config(
        prompt: impl Into<String>,
        model: &OpenAICompletionModel,
        config: CompletionConfig,
    ) -> Result<Self, ModelError> {
        let request = Client::new()
            .post(model.config.get_url())
            .header(AUTHORIZATION, format!("Bearer {}", model.api_key))
            .json(&CompletionBody {
                prompt: prompt.into(),
                stream: Some(true),
                config,
            });

        let event_src = request
            .eventsource()
            .map_err(|_| OpenAIError::CannotCloneRequestError)?;

        Ok(CompletionModelStream::new(event_src))
    }
}

impl<M> Default for OpenAI<M>
where
    M: ModelType,
{
    fn default() -> Self {
        Self::new(Default::default(), env::var("OPENAI_API_KEY").unwrap())
    }
}

//-------------------------------------------------------------------------------------------------
// Tests
//-------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn language_model_config_defaults_are_correct() {
        let model = OpenAIModel::default();

        assert_eq!(model.config.model, ChatModel::GPT3_5Turbo);
        assert_eq!(model.config.attributes.suffix, None);
        assert_eq!(model.config.attributes.max_tokens, None);
        assert_eq!(model.config.attributes.temperature, None);
        assert_eq!(model.config.attributes.top_p, None);
        assert_eq!(model.config.attributes.n, None);
        assert_eq!(model.config.attributes.stream, None);
        assert_eq!(model.config.attributes.logprobs, None);
        assert_eq!(model.config.attributes.echo, None);
        assert_eq!(model.config.attributes.stop, None);
        assert_eq!(model.config.attributes.presence_penalty, None);
        assert_eq!(model.config.attributes.frequency_penalty, None);
        assert_eq!(model.config.attributes.best_of, None);
        assert_eq!(model.config.attributes.logit_bias, None);
        assert_eq!(model.config.attributes.user, None);
    }
}
