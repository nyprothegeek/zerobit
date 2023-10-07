use std::fmt::Display;

use serde::Deserialize;
use thiserror::Error;

//-------------------------------------------------------------------------------------------------
// Types
//-------------------------------------------------------------------------------------------------

#[derive(Debug, Error)]
pub enum OpenAIError {
    #[error("api: {0}")]
    API(APIError),

    #[error("reqwest: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("eventsource: {0}")]
    EventSource(#[from] reqwest_eventsource::Error),

    #[error("serde_json: {0}")]
    SerdeJson(#[from] serde_json::Error),

    #[error("cannot clone request")]
    CannotCloneRequestError,

    #[error("completion missing from response")]
    CompletionMissing,

    #[error("missing api key")]
    MissingAPIKey,
}

#[derive(Debug, Deserialize, Error)]
pub struct APIError {
    pub code: Option<String>,
    pub message: String,
    pub param: Option<String>,
    pub r#type: String,
}

//-------------------------------------------------------------------------------------------------
// Trait Implementations
//-------------------------------------------------------------------------------------------------

impl Display for APIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("APIError")
            .field("code", &self.code)
            .field("message", &self.message)
            .field("param", &self.param)
            .field("type", &self.r#type)
            .finish()?;

        Ok(())
    }
}
