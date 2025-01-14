use thiserror::Error;
use versa_model::ModelError;

//-------------------------------------------------------------------------------------------------
// Types
//-------------------------------------------------------------------------------------------------

#[derive(Debug, Error)]
pub enum ChainError {
    #[error("model: {0}")]
    ModelError(#[from] ModelError),
}
