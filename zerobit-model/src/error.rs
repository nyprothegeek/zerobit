use thiserror::Error;

//-------------------------------------------------------------------------------------------------
// Types
//-------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Error)]
pub enum ModelError {
    // #[error("OpenAI API error: {0}")]
    // OpenAIAPI(#[from] reqwest::Error),
}
