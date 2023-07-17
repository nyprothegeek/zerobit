use std::collections::HashMap;
use zerobit_common::pattern::Pattern;

use crate::{PromptError, Tags};

//-------------------------------------------------------------------------------------------------
// Traits
//-------------------------------------------------------------------------------------------------

pub trait ResolvablePrompt {
    type ResolvedType: ResolvedPrompt;

    /// Checks if the prompt has unresolved variables.
    fn has_unresolved_vars(&self) -> Result<bool, PromptError>;

    /// Replaces all occurrences of a given variable with the provided value.
    fn resolve_var(&mut self, var: &str, value: &str) -> Result<(), PromptError>;

    /// Changes occurences of the given variables to the given values if they exist.
    fn format<'a>(&mut self, map: HashMap<&'a str, &'a str>) -> Result<(), PromptError>;

    /// Resolves the prompt.
    fn resolve(self) -> Result<Self::ResolvedType, PromptError>;
}

pub trait ResolvedPrompt {
    /// Function for selecting prompt based on a pattern.
    /// If pattern is not found, returns all prompts.
    fn get_prompt_by_pattern(&self, pattern: Pattern) -> Result<Vec<(String, Tags)>, PromptError>;
}
