//! This module contains implementations of OpenAI models.

use std::{env, fmt::Debug};

use super::{ChatConfig, ChatModel, CompletionModel, ModelType, OpenAIConfig, OutputStream};
use crate::traits::{Model, Output};
use anyhow::Result;
use async_trait::async_trait;
use derivative::Derivative;
use reqwest::{header::AUTHORIZATION, Client};
use serde::{Deserialize, Serialize};
use strum_macros::Display;

//-------------------------------------------------------------------------------------------------
// Aliases
//-------------------------------------------------------------------------------------------------

pub type StringStream = OutputStream<String>;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatChoice {
    pub message: ChatMessage,
    pub index: u8,
    pub finish_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionChoice {
    pub index: u8,
    pub text: String,
    pub logprobs: Option<u8>,
    pub finish_reason: String,
}

#[derive(Debug, Serialize, Deserialize, Display)]
pub enum ChatRole {
    #[serde(rename = "system")]
    #[strum(serialize = "system")]
    System,

    #[serde(rename = "user")]
    #[strum(serialize = "user")]
    User,

    #[serde(rename = "assistant")]
    #[strum(serialize = "assistant")]
    Assistant,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: ChatRole,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelResponse<T> {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<T>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ChatBody {
    pub messages: Vec<ChatMessage>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    #[serde(flatten)]
    pub attributes: ChatConfig,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CompletionBody {
    pub prompt: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    #[serde(flatten)]
    pub attributes: ChatConfig,
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
    pub async fn get_completion(&self, _messages: Vec<ChatMessage>) -> Result<ChatModelResponse> {
        todo!()
    }
}

impl OpenAICompletionModel {
    // TODO(nyprothegeek): Support stream.
    /// Sends a request to the OpenAI API to get a completion.
    pub async fn get_completion(
        &self,
        _prompt: impl Into<String>,
    ) -> Result<CompletionModelResponse> {
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
    type Error = anyhow::Error;

    async fn prompt<O>(&self, prompt: impl Into<String>) -> Result<O>
    where
        O: Output<Self> + Debug,
    {
        let output = O::from_call(prompt, self).await?;

        #[cfg(feature = "log")]
        log::debug!("output = {output:?}");

        Ok(output)
    }
}

#[async_trait(?Send)]
impl Output<OpenAIChatModel> for String {
    async fn from_call(prompt: impl Into<String>, model: &OpenAIChatModel) -> Result<Self> {
        let request = Client::new()
            .post(model.config.get_url())
            .header(AUTHORIZATION, format!("Bearer {}", model.api_key))
            .json(&ChatBody {
                messages: vec![ChatMessage {
                    content: prompt.into(),
                    role: ChatRole::User,
                }],
                ..Default::default()
            });

        // let response = request.send().await?.json::<ChatModelResponse>().await?;

        // Ok(response.choices[0].message.content.clone());

        // TODO(nyprothegeek): Handle errors.
        let text_response = request.send().await?.text().await?;

        Ok(text_response)
    }
}

#[async_trait(?Send)]
impl Output<OpenAICompletionModel> for String {
    async fn from_call(prompt: impl Into<String>, model: &OpenAICompletionModel) -> Result<Self> {
        let request = Client::new()
            .post(model.config.get_url())
            .header(AUTHORIZATION, format!("Bearer {}", model.api_key))
            .json(&CompletionBody {
                prompt: prompt.into(),
                ..Default::default()
            });

        // let response = request
        //     .send()
        //     .await?
        //     .json::<CompletionModelResponse>()
        //     .await?;

        // Ok(response.choices[0].text.clone())

        // TODO(nyprothegeek): Handle errors.
        let text_response = request.send().await?.text().await?;

        Ok(text_response)
    }
}

// #[async_trait(?Send)]
// impl OpenAIChatModel for StringStream {
//     async fn from_call(prompt: impl Into<String>, model: &OpenAIChatModel) -> Result<Self> {
//         // Set stream in body.
//         // let request_stream = Client::new() ... .eventsource().await?;
//         todo!()
//     }
// }

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
    use crate::openai::{Attributes, ChatConfig, CompletionConfig};

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

    #[tokio::test]
    async fn test_0() {
        let model = OpenAICompletionModel::with_config(CompletionConfig {
            model: CompletionModel::Davinci,
            attributes: Attributes {
                temperature: Some(0.5),
                ..Default::default()
            },
        });

        println!("model = {model:#?}");

        let model = OpenAIChatModel::with_config(ChatConfig {
            model: ChatModel::GPT3_5Turbo16k,
            ..Default::default()
        });

        println!("model = {model:#?}");
    }

    #[tokio::test]
    async fn test_1() -> Result<()> {
        let model = OpenAIChatModel::default();
        let response: String = model.prompt("Hello there!").await?;
        println!("{response:#?}");

        Ok(())
    }
}
