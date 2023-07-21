use crate::{PromptError, ResolvablePrompt, ResolvedPrompt};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, HashMap},
    fmt::Debug,
};
use versa_common::{pattern::Pattern, traits::Config};

//-------------------------------------------------------------------------------------------------
// Aliases
//-------------------------------------------------------------------------------------------------

pub type Tags = Vec<Tag>;
pub type Prompt = PromptData<String>;
pub type PromptMap = PromptData<BTreeMap<(usize, String), Tags>>;
pub type ResolvedPromptString = ResolvedPromptData<String>;
pub type ResolvedPromptMap = ResolvedPromptData<BTreeMap<(usize, String), Tags>>;

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

/// An iterator over the messages of a `Prompt`.
pub struct Iter<'a> {
    iter: std::collections::btree_map::Iter<'a, (usize, String), Tags>,
}

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

    /// Gets the message of the prompt.
    pub fn get_message(&self) -> &str {
        &self.data
    }
}

impl PromptMap {
    /// Creates a new prompt.
    pub fn new(message: impl Into<String>, tags: Vec<Tag>) -> Self {
        Self {
            data: vec![((0, message.into()), tags)].into_iter().collect(),
        }
    }

    /// Adds a message to the prompt.
    pub fn add_message(&mut self, message: impl Into<String>, tags: Vec<Tag>) {
        self.data
            .entry((self.data.len(), message.into()))
            .or_default()
            .extend(tags);
    }

    /// Returns an iterator over the messages of the prompt.
    pub fn iter(&self) -> Iter {
        Iter {
            iter: self.data.iter(),
        }
    }
}

//-------------------------------------------------------------------------------------------------
// Trait Implementations
//-------------------------------------------------------------------------------------------------

impl<'a> Iterator for Iter<'a> {
    type Item = ((&'a usize, &'a str), &'a [Tag]);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|((index, message), tags)| ((index, message.as_str()), tags.as_slice()))
    }
}

impl Config for Prompt {}

impl Config for PromptMap {}

impl ResolvedPrompt for ResolvedPromptString {
    fn get_prompt_by_pattern(&self, _: Pattern) -> Result<Vec<(String, Tags)>, PromptError> {
        Ok(vec![(self.0.data.clone(), vec![])])
    }
}

impl ResolvedPrompt for ResolvedPromptMap {
    fn get_prompt_by_pattern(&self, _: Pattern) -> Result<Vec<(String, Tags)>, PromptError> {
        // TODO(nyprothegeek): Use the pattern to select the right prompt messages.
        let mut prompts = vec![];
        for ((_, message), tags) in self.0.iter() {
            prompts.push((message.into(), tags.into()));
        }
        Ok(prompts)
    }
}

impl ResolvablePrompt for Prompt {
    type ResolvedType = ResolvedPromptString;

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

    fn resolve(self) -> Result<Self::ResolvedType, PromptError> {
        if self.has_unresolved_vars()? {
            return Err(PromptError::UnresolvedVars);
        }

        Ok(ResolvedPromptData(self))
    }
}

impl ResolvablePrompt for PromptMap {
    type ResolvedType = ResolvedPromptMap;

    fn has_unresolved_vars(&self) -> Result<bool, PromptError> {
        let re = Regex::new(r"\{\{(?<var>[a-zA-Z_][a-zA-Z0-9_]*)\}\}")?;
        for (_, message) in self.data.keys() {
            if re.is_match(message) {
                return Ok(true);
            }
        }

        Ok(false)
    }

    fn resolve_var(&mut self, var: &str, value: &str) -> Result<(), PromptError> {
        let re = Regex::new(&format!(r"\{{\{{(?<var>{})\}}\}}", var))?;
        let messages = std::mem::take(&mut self.data);
        for ((index, message), tags) in messages {
            self.data
                .insert((index, re.replace_all(&message, value).into()), tags);
        }

        Ok(())
    }

    fn format(&mut self, map: HashMap<&str, &str>) -> Result<(), PromptError> {
        for (var, value) in map {
            self.resolve_var(var, value)?;
        }
        Ok(())
    }

    fn resolve(self) -> Result<Self::ResolvedType, PromptError> {
        if self.has_unresolved_vars()? {
            return Err(PromptError::UnresolvedVars);
        }

        Ok(ResolvedPromptData(self))
    }
}

impl ResolvablePrompt for String {
    type ResolvedType = ResolvedPromptString;

    fn has_unresolved_vars(&self) -> Result<bool, PromptError> {
        let re = Regex::new(r"\{\{(?<var>[a-zA-Z_][a-zA-Z0-9_]*)\}\}")?;
        if re.is_match(self) {
            return Ok(true);
        }

        Ok(false)
    }

    fn resolve_var(&mut self, var: &str, value: &str) -> Result<(), PromptError> {
        let re = Regex::new(&format!(r"\{{\{{(?<var>{})\}}\}}", var))?;
        *self = re.replace_all(self, value).into();
        Ok(())
    }

    fn format(&mut self, map: HashMap<&str, &str>) -> Result<(), PromptError> {
        for (var, value) in map {
            self.resolve_var(var, value)?;
        }
        Ok(())
    }

    fn resolve(self) -> Result<Self::ResolvedType, PromptError> {
        if self.has_unresolved_vars()? {
            return Err(PromptError::UnresolvedVars);
        }

        Ok(ResolvedPromptData(Prompt::new(self)))
    }
}

/// This one exists for convenience. Some of its methods are not implemented.
/// This is because it is not possible to implement them for `&str`.
impl ResolvablePrompt for &'static str {
    type ResolvedType = ResolvedPromptString;

    fn has_unresolved_vars(&self) -> Result<bool, PromptError> {
        let re = Regex::new(r"\{\{(?<var>[a-zA-Z_][a-zA-Z0-9_]*)\}\}")?;
        if re.is_match(self) {
            return Ok(true);
        }

        Ok(false)
    }

    fn resolve_var(&mut self, _: &str, _: &str) -> Result<(), PromptError> {
        unimplemented!()
    }

    fn format(&mut self, _: HashMap<&str, &str>) -> Result<(), PromptError> {
        unimplemented!()
    }

    fn resolve(self) -> Result<Self::ResolvedType, PromptError> {
        if self.has_unresolved_vars()? {
            return Err(PromptError::UnresolvedVars);
        }

        Ok(ResolvedPromptData(Prompt::new(self)))
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

        let prompt = PromptMap::new(
            "You are a helpful AI assistant that help users to {{job}}!",
            vec![Tag::Role(Role::System)],
        );
        assert!(prompt.has_unresolved_vars().unwrap());
    }

    #[test]
    fn test_resolved_prompt_conversion_succeeds() {
        let prompt = PromptMap::new("Hello World!", vec![]);
        assert!(!prompt.has_unresolved_vars().unwrap());

        let prompt = PromptMap::new("Hello {{name!", vec![]);
        assert!(!prompt.has_unresolved_vars().unwrap());

        let result = prompt.resolve();
        assert!(result.is_ok());
    }

    #[test]
    fn test_can_resolve_variables() {
        let mut prompt = Prompt::new("Hello {{name}}! What is your {{question}}?");
        prompt
            .format(map!("name" => "ChatGPT", "question" => "favorite color"))
            .unwrap();

        assert_eq!(
            prompt.get_message(),
            "Hello ChatGPT! What is your favorite color?"
        );

        let mut prompt = PromptMap::new("Hello {{name}}! What is your {{question}}?", vec![]);
        prompt
            .format(map!("name" => "ChatGPT", "question" => "favorite color"))
            .unwrap();

        let ((_, message), _) = prompt.iter().next().unwrap();
        assert_eq!(message, "Hello ChatGPT! What is your favorite color?");
    }
}
