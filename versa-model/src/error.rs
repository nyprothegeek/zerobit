use crate::openai::OpenAIError;
use thiserror::Error;

//-------------------------------------------------------------------------------------------------
// Types
//-------------------------------------------------------------------------------------------------

#[derive(Debug, Error)]
pub enum ModelError {
    #[error("openai: {0}")]
    OpenAI(#[from] OpenAIError),
}
