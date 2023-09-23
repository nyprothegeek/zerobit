use crate::PromptError;
use std::collections::HashMap;

//-------------------------------------------------------------------------------------------------
// Traits
//-------------------------------------------------------------------------------------------------

pub trait FinalizablePrompt {
    type FinalizedPrompt: FinalizedPrompt;

    /// Checks if the prompt has unresolved variables.
    fn has_unresolved_vars(&self) -> Result<bool, PromptError>;

    /// Replaces all occurrences of a given variable with the provided value.
    fn resolve_var(&mut self, var: &str, value: &str) -> Result<(), PromptError>;

    /// Changes occurences of the given variables to the given values if they exist.
    fn format(&mut self, map: HashMap<&str, &str>) -> Result<(), PromptError>;

    /// Resolves into its final form.
    fn finalize(self) -> Result<Self::FinalizedPrompt, PromptError>;

    /// Resolves the prompt into a finalized prompt.
    fn resolve(mut self, map: HashMap<&str, &str>) -> Result<Self::FinalizedPrompt, PromptError>
    where
        Self: Sized,
    {
        self.format(map)?;
        if self.has_unresolved_vars()? {
            return Err(PromptError::UnresolvedVars);
        }

        self.finalize()
    }
}

pub trait FinalizedPrompt {}
