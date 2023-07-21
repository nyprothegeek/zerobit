use crate::openai::OpenAIError;
use thiserror::Error;
use versa_prompt::PromptError;

//-------------------------------------------------------------------------------------------------
// Types
//-------------------------------------------------------------------------------------------------

#[derive(Debug, Error)]
pub enum ModelError {
    #[error("openai: {0}")]
    OpenAI(#[from] OpenAIError),

    #[error("prompt: {0}")]
    Prompt(#[from] PromptError),
}
