use thiserror::Error;

//-------------------------------------------------------------------------------------------------
// Types
//-------------------------------------------------------------------------------------------------

#[derive(Debug, Error)]
pub enum PromptError {
    #[error("Prompt contains unresolved variables.")]
    UnresolvedVars,

    #[error("Regex error: {0}")]
    RegexError(#[from] regex::Error),
}
