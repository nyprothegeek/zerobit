//! This module contains implementations of OpenAI models.

use super::{
    ChatConfig, ChatMessage, ChatMessages, ChatModel, ChatModelStream, CompletionConfig,
    CompletionModel, CompletionModelStream, ModelKind, OpenAIConfig,
};
use crate::{
    openai::{APIError, OpenAIError},
    traits::{Model, Output},
    ModelError,
};
use async_trait::async_trait;
use reqwest::{header::AUTHORIZATION, Client};
use reqwest_eventsource::RequestBuilderExt;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    env,
    fmt::{self, Debug, Formatter},
};
use versa_common::traits::Config;

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

// TODO(nyprothegeek): Implement call functions on the models taking the and returning the OpenAI data types.
/// An OpenAI language model.
#[derive(Serialize, Deserialize, Clone)]
pub struct OpenAI<M>
where
    M: ModelKind,
{
    /// The configuration of the model.
    #[serde(flatten)]
    config: M::Config,

    // OpenAI API key.
    #[serde(skip)]
    api_key: Option<String>,
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
    pub messages: ChatMessages,

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

//-------------------------------------------------------------------------------------------------
// Methods
//-------------------------------------------------------------------------------------------------

// TODO(nyprothegeek): Document the builder methods properly.
impl<M> OpenAI<M>
where
    M: ModelKind,
{
    /// Creates a new OpenAI model with the given configuration.
    pub fn with_config(config: M::Config) -> Self {
        Self {
            config,
            api_key: Default::default(),
        }
    }

    /// Sets the API key.
    pub fn api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }
}

// TODO(nyprothegeek): Document the builder methods properly.
impl OpenAIChatModel {
    /// Sets the model.
    pub fn model(mut self, model: ChatModel) -> Self {
        self.config.model = model;
        self
    }

    /// Sets the suffix.
    pub fn suffix(mut self, suffix: impl Into<String>) -> Self {
        self.config.attributes.suffix = Some(suffix.into());
        self
    }

    /// Sets the max tokens.
    pub fn max_tokens(mut self, max_tokens: u16) -> Self {
        self.config.attributes.max_tokens = Some(max_tokens);
        self
    }

    /// Sets the temperature.
    pub fn temperature(mut self, temperature: f32) -> Self {
        self.config.attributes.temperature = Some(temperature);
        self
    }

    /// Sets the top p.
    pub fn top_p(mut self, top_p: f32) -> Self {
        self.config.attributes.top_p = Some(top_p);
        self
    }

    /// Sets the n.
    pub fn n(mut self, n: u8) -> Self {
        self.config.attributes.n = Some(n);
        self
    }

    /// Sets the logprobs.
    pub fn logprobs(mut self, logprobs: u8) -> Self {
        self.config.attributes.logprobs = Some(logprobs);
        self
    }

    /// Sets the echo.
    pub fn echo(mut self, echo: bool) -> Self {
        self.config.attributes.echo = Some(echo);
        self
    }

    /// Sets the presence penalty.
    pub fn presence_penalty(mut self, presence_penalty: f32) -> Self {
        self.config.attributes.presence_penalty = Some(presence_penalty);
        self
    }

    /// Sets the frequency penalty.
    pub fn frequency_penalty(mut self, frequency_penalty: f32) -> Self {
        self.config.attributes.frequency_penalty = Some(frequency_penalty);
        self
    }

    /// Sets the best of.
    pub fn best_of(mut self, best_of: u8) -> Self {
        self.config.attributes.best_of = Some(best_of);
        self
    }

    /// Sets the logit bias.
    pub fn logit_bias(mut self, logit_bias: HashMap<u64, i8>) -> Self {
        self.config.attributes.logit_bias = Some(logit_bias);
        self
    }

    /// Sets the user token.
    pub fn user(mut self, user_token: impl Into<String>) -> Self {
        self.config.attributes.user = Some(user_token.into());
        self
    }
}

// TODO(nyprothegeek): Document the builder methods properly.
impl OpenAICompletionModel {
    /// Sets the model.
    pub fn model(mut self, model: CompletionModel) -> Self {
        self.config.model = model;
        self
    }

    /// Sets the suffix.
    pub fn suffix(mut self, suffix: impl Into<String>) -> Self {
        self.config.attributes.suffix = Some(suffix.into());
        self
    }

    /// Sets the max tokens.
    pub fn max_tokens(mut self, max_tokens: u16) -> Self {
        self.config.attributes.max_tokens = Some(max_tokens);
        self
    }

    /// Sets the temperature.
    pub fn temperature(mut self, temperature: f32) -> Self {
        self.config.attributes.temperature = Some(temperature);
        self
    }

    /// Sets the top p.
    pub fn top_p(mut self, top_p: f32) -> Self {
        self.config.attributes.top_p = Some(top_p);
        self
    }

    /// Sets the n.
    pub fn n(mut self, n: u8) -> Self {
        self.config.attributes.n = Some(n);
        self
    }

    /// Sets the logprobs.
    pub fn logprobs(mut self, logprobs: u8) -> Self {
        self.config.attributes.logprobs = Some(logprobs);
        self
    }

    /// Sets the echo.
    pub fn echo(mut self, echo: bool) -> Self {
        self.config.attributes.echo = Some(echo);
        self
    }

    /// Sets the presence penalty.
    pub fn presence_penalty(mut self, presence_penalty: f32) -> Self {
        self.config.attributes.presence_penalty = Some(presence_penalty);
        self
    }

    /// Sets the frequency penalty.
    pub fn frequency_penalty(mut self, frequency_penalty: f32) -> Self {
        self.config.attributes.frequency_penalty = Some(frequency_penalty);
        self
    }

    /// Sets the best of.
    pub fn best_of(mut self, best_of: u8) -> Self {
        self.config.attributes.best_of = Some(best_of);
        self
    }

    /// Sets the logit bias.
    pub fn logit_bias(mut self, logit_bias: HashMap<u64, i8>) -> Self {
        self.config.attributes.logit_bias = Some(logit_bias);
        self
    }

    /// Sets the user token.
    pub fn user(mut self, user_token: impl Into<String>) -> Self {
        self.config.attributes.user = Some(user_token.into());
        self
    }
}

//-------------------------------------------------------------------------------------------------
// Trait Implementations
//-------------------------------------------------------------------------------------------------

#[async_trait(?Send)]
impl<M> Model for OpenAI<M>
where
    M: ModelKind,
{
    type Config = M::Config;
    type Input = M::Input;

    async fn prompt<O>(&self, input: impl Into<Self::Input>) -> Result<O, ModelError>
    where
        O: Output<Self>,
    {
        O::from_call(input, self).await
    }

    async fn prompt_with_config<O>(
        &self,
        input: impl Into<Self::Input>,
        config: Self::Config,
    ) -> Result<O, ModelError>
    where
        O: Output<Self>,
    {
        O::from_call_with_config(input, self, config).await
    }

    fn get_config(&self) -> &Self::Config {
        &self.config
    }
}

#[async_trait(?Send)]
impl Output<OpenAIChatModel> for String {
    async fn from_call_with_config(
        input: impl Into<ChatMessages>,
        model: &OpenAIChatModel,
        config: ChatConfig,
    ) -> Result<Self, ModelError> {
        let request = Client::new()
            .post(model.config.get_url())
            .header(
                AUTHORIZATION,
                format!(
                    "Bearer {}",
                    model.api_key.as_ref().ok_or(OpenAIError::MissingAPIKey)?
                ),
            )
            .json(&ChatBody {
                messages: input.into(),
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
        input: impl Into<String>,
        model: &OpenAICompletionModel,
        config: CompletionConfig,
    ) -> Result<Self, ModelError> {
        let request = Client::new()
            .post(model.config.get_url())
            .header(
                AUTHORIZATION,
                format!(
                    "Bearer {}",
                    model.api_key.as_ref().ok_or(OpenAIError::MissingAPIKey)?
                ),
            )
            .json(&CompletionBody {
                prompt: input.into(),
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
        input: impl Into<ChatMessages>,
        model: &OpenAIChatModel,
        config: ChatConfig,
    ) -> Result<Self, ModelError> {
        let request = Client::new()
            .post(model.config.get_url())
            .header(
                AUTHORIZATION,
                format!(
                    "Bearer {}",
                    model.api_key.as_ref().ok_or(OpenAIError::MissingAPIKey)?
                ),
            )
            .json(&ChatBody {
                messages: input.into(),
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
        input: impl Into<String>,
        model: &OpenAICompletionModel,
        config: CompletionConfig,
    ) -> Result<Self, ModelError> {
        let request = Client::new()
            .post(model.config.get_url())
            .header(
                AUTHORIZATION,
                format!(
                    "Bearer {}",
                    model.api_key.as_ref().ok_or(OpenAIError::MissingAPIKey)?
                ),
            )
            .json(&CompletionBody {
                prompt: input.into(),
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
    M: ModelKind,
{
    fn default() -> Self {
        Self {
            config: Default::default(),
            api_key: Some(env::var("OPENAI_API_KEY").unwrap()),
        }
    }
}

impl<M> Debug for OpenAI<M>
where
    M: ModelKind,
    M::Config: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("OpenAI")
            .field("config", &self.config)
            .finish()
    }
}

impl<M> Config for OpenAI<M> where M: ModelKind {}

//-------------------------------------------------------------------------------------------------
// Tests
//-------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use versa_common::{utils, Env};

    use super::*;

    #[test]
    fn language_model_config_defaults_are_correct() {
        utils::load_env(Env::Test);
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

        let model = OpenAICompletionModel::default();

        assert_eq!(model.config.model, CompletionModel::TextDaVinci003);
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
