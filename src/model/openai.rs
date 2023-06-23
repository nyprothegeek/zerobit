//! # OpenAI
//!
//! This module contains implementations of OpenAI models.

use super::Model;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

//-------------------------------------------------------------------------------------------------
// Types
//-------------------------------------------------------------------------------------------------

/// An OpenAI language model.
#[derive(Debug, Builder, Serialize, Deserialize)]
pub struct OpenAiLangModel {
    #[serde(skip)]
    /// The API key used to access the model.
    pub api_key: String,
    /// The name of the model.
    pub model: String,
    /// The temperature of the model.
    pub temperature: f32,
    /// The maximum number of tokens to generate.
    pub max_tokens: u32,
    /// The probability of top tokens to consider.
    pub top_p: f32,
    /// The number of tokens to generate.
    pub stop: Vec<String>,
}

//-------------------------------------------------------------------------------------------------
// Methods
//-------------------------------------------------------------------------------------------------

// impl OpenAiLangModel {}

//-------------------------------------------------------------------------------------------------
// Trait Implementations
//-------------------------------------------------------------------------------------------------

impl Model for OpenAiLangModel {
    type Input = String;
    type Output = String;

    fn generate(&self, _prompt: Self::Input) -> Self::Output {
        unimplemented!()
    }
}

//-------------------------------------------------------------------------------------------------
// Tests
//-------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        unimplemented!()
    }
}
