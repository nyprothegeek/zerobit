use serde::{Deserialize, Serialize};
use strum_macros::Display;
use versa_prompt::{FinalPrompt, FinalPromptList, Role, Tag};

//-------------------------------------------------------------------------------------------------
// Types
//-------------------------------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize, Display)]
#[serde(rename_all = "snake_case")]
pub enum ChatRole {
    #[strum(serialize = "system")]
    System,
    #[strum(serialize = "user")]
    User,
    #[strum(serialize = "assistant")]
    Assistant,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: ChatRole,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct ChatStreamMessage {
    pub role: Option<ChatRole>,
    pub content: Option<String>,
}

#[derive(Debug, Serialize, Default)]
pub struct ChatMessages(Vec<ChatMessage>);

//-------------------------------------------------------------------------------------------------
// Trait Implementations
//-------------------------------------------------------------------------------------------------

impl From<Vec<ChatMessage>> for ChatMessages {
    fn from(v: Vec<ChatMessage>) -> Self {
        Self(v)
    }
}

impl From<String> for ChatMessages {
    fn from(s: String) -> Self {
        Self(vec![ChatMessage {
            role: ChatRole::User,
            content: s,
        }])
    }
}

impl From<&str> for ChatMessages {
    fn from(s: &str) -> Self {
        Self(vec![ChatMessage {
            role: ChatRole::User,
            content: s.to_string(),
        }])
    }
}

impl From<ChatMessages> for Vec<ChatMessage> {
    fn from(cm: ChatMessages) -> Self {
        cm.0
    }
}

impl From<FinalPromptList> for ChatMessages {
    fn from(list: FinalPromptList) -> Self {
        let mut messages = vec![];
        for (content, tags) in list.into_iter() {
            let role = tags
                .into_iter()
                .find_map(|tag| match tag {
                    Tag::Role(role) => match role {
                        Role::System => Some(ChatRole::System),
                        Role::User => Some(ChatRole::User),
                        Role::Assistant => Some(ChatRole::Assistant),
                    },
                    _ => None,
                })
                .unwrap_or(ChatRole::User);

            messages.push(ChatMessage { role, content });
        }
        Self(messages)
    }
}

impl From<FinalPrompt> for ChatMessages {
    fn from(prompt: FinalPrompt) -> Self {
        Self(vec![ChatMessage {
            role: ChatRole::User,
            content: prompt.into(),
        }])
    }
}
