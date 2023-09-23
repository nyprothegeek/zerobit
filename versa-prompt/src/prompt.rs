use crate::{FinalizablePrompt, FinalizedPrompt, PromptError};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Debug, vec};
use versa_common::{pattern::Pattern, traits::Config};

//-------------------------------------------------------------------------------------------------
// Aliases
//-------------------------------------------------------------------------------------------------

pub type Tags = Vec<Tag>;
pub type Prompt = PromptData<String>;
pub type PromptList = PromptData<Vec<(String, Tags)>>;
pub type ResolvedPrompt = ResolvedPromptData<String>;
pub type ResolvedPromptList = ResolvedPromptData<Vec<(String, Tags)>>;

//-------------------------------------------------------------------------------------------------
// Types
//-------------------------------------------------------------------------------------------------

/// A prompt is a message that is sent to a model.
///
/// A prompt can either be resolved or unresolved.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct PromptData<T> {
    data: T,
}

/// A resolved prompt is a prompt that has been resolved.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResolvedPromptData<T>(PromptData<T>);

/// A tag that describes the prompt message identity.
/// This becomes useful later when converting prompts to strings.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Tag {
    Pattern(Pattern),
    Role(Role),
}

/// A role is often used by chat models to classify the prompt message.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
}

/// An iterator over the messages of a `PromptList`.
pub struct PromptListIter<'a> {
    iter: std::slice::Iter<'a, (String, Tags)>,
}

//-------------------------------------------------------------------------------------------------
// Methods
//-------------------------------------------------------------------------------------------------

impl Prompt {
    /// Creates a new prompt.
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            data: message.into(),
        }
    }
}

impl PromptList {
    /// Creates a new prompt.
    pub fn new(message: impl Into<String>, tags: Vec<Tag>) -> Self {
        Self {
            data: vec![(message.into(), tags)],
        }
    }

    /// Adds a message to the prompt.
    pub fn add_message(&mut self, message: impl Into<String>, tags: Vec<Tag>) {
        self.data.push((message.into(), tags));
    }

    /// Returns an iterator over the messages of the prompt.
    pub fn iter(&self) -> PromptListIter {
        PromptListIter {
            iter: self.data.iter(),
        }
    }
}

impl ResolvedPromptList {
    /// Returns an iterator over the messages of the prompt.
    pub fn iter(&self) -> PromptListIter {
        PromptListIter {
            iter: self.0.data.iter(),
        }
    }
}

//-------------------------------------------------------------------------------------------------
// Trait Implementations
//-------------------------------------------------------------------------------------------------

impl FinalizablePrompt for Prompt {
    type FinalizedPrompt = ResolvedPrompt;

    fn has_unresolved_vars(&self) -> Result<bool, PromptError> {
        let re = Regex::new(r"\{\{(?<var>[a-zA-Z_][a-zA-Z0-9_]*)\}\}")?;
        if re.is_match(&self.data) {
            return Ok(true);
        }

        Ok(false)
    }

    fn resolve_var(&mut self, var: &str, value: &str) -> Result<(), PromptError> {
        let re = Regex::new(&format!(r"\{{\{{(?<var>{})\}}\}}", var))?;
        self.data = re.replace_all(&self.data, value).into();
        Ok(())
    }

    fn format(&mut self, map: HashMap<&str, &str>) -> Result<(), PromptError> {
        for (var, value) in map {
            self.resolve_var(var, value)?;
        }
        Ok(())
    }

    fn finalize(self) -> Result<Self::FinalizedPrompt, PromptError> {
        if self.has_unresolved_vars()? {
            return Err(PromptError::UnresolvedVars);
        }

        Ok(ResolvedPromptData(self))
    }
}

impl FinalizablePrompt for PromptList {
    type FinalizedPrompt = ResolvedPromptList;

    fn has_unresolved_vars(&self) -> Result<bool, PromptError> {
        let re = Regex::new(r"\{\{(?<var>[a-zA-Z_][a-zA-Z0-9_]*)\}\}")?;
        for (message, _) in self.data.iter() {
            if re.is_match(message) {
                return Ok(true);
            }
        }

        Ok(false)
    }

    fn resolve_var(&mut self, var: &str, value: &str) -> Result<(), PromptError> {
        let re = Regex::new(&format!(r"\{{\{{(?<var>{})\}}\}}", var))?;
        let messages = std::mem::take(&mut self.data);
        for (message, tags) in messages {
            self.data
                .push((re.replace_all(&message, value).into(), tags));
        }

        Ok(())
    }

    fn format(&mut self, map: HashMap<&str, &str>) -> Result<(), PromptError> {
        for (var, value) in map {
            self.resolve_var(var, value)?;
        }
        Ok(())
    }

    fn finalize(self) -> Result<Self::FinalizedPrompt, PromptError> {
        if self.has_unresolved_vars()? {
            return Err(PromptError::UnresolvedVars);
        }

        Ok(ResolvedPromptData(self))
    }
}

impl IntoIterator for PromptList {
    type Item = (String, Vec<Tag>);
    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'a> Iterator for PromptListIter<'a> {
    type Item = &'a (String, Vec<Tag>);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl IntoIterator for ResolvedPromptList {
    type Item = (String, Vec<Tag>);
    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.data.into_iter()
    }
}

impl Config for Prompt {}

impl Config for PromptList {}

impl FinalizedPrompt for ResolvedPrompt {}

impl FinalizedPrompt for ResolvedPromptList {}

impl From<ResolvedPromptList> for String {
    fn from(_: ResolvedPromptList) -> Self {
        // TODO(nyprothegeek): Combine all the messages into one with roles added.
        todo!()
    }
}

impl From<Prompt> for String {
    fn from(prompt: Prompt) -> Self {
        prompt.data
    }
}

impl From<ResolvedPrompt> for String {
    fn from(prompt: ResolvedPrompt) -> Self {
        prompt.0.data
    }
}

//-------------------------------------------------------------------------------------------------
// Tests
//-------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::map;
    use std::vec;

    #[test]
    fn test_can_check_unresolved_prompts() {
        let prompt = Prompt::new("Hello {{name}}!");
        assert!(prompt.has_unresolved_vars().unwrap());

        let prompt = PromptList::new(
            "You are a helpful AI assistant that help users to {{job}}!",
            vec![Tag::Role(Role::System)],
        );
        assert!(prompt.has_unresolved_vars().unwrap());
    }

    #[test]
    fn test_resolved_prompt_conversion_succeeds() {
        let prompt = PromptList::new("Hello World!", vec![]);
        assert!(!prompt.has_unresolved_vars().unwrap());

        let prompt = PromptList::new("Hello {{name!", vec![]);
        assert!(!prompt.has_unresolved_vars().unwrap());

        let result = prompt.finalize();
        assert!(result.is_ok());
    }

    #[test]
    fn test_can_resolve_variables() {
        let mut prompt = Prompt::new("Hello {{name}}! What is your {{question}}?");
        prompt
            .format(map!("name" => "ChatGPT", "question" => "favorite color"))
            .unwrap();

        assert_eq!(
            String::from(prompt),
            "Hello ChatGPT! What is your favorite color?"
        );

        let mut prompt = PromptList::new("Hello {{name}}! What is your {{question}}?", vec![]);
        prompt
            .format(map!("name" => "ChatGPT", "question" => "favorite color"))
            .unwrap();

        let (message, _) = prompt.iter().next().unwrap();
        assert_eq!(message, "Hello ChatGPT! What is your favorite color?");
    }
}
