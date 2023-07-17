use serde::{Deserialize, Serialize};
use std::fmt::Debug;

//-------------------------------------------------------------------------------------------------
// Traits
//-------------------------------------------------------------------------------------------------

pub trait Config: Serialize + Deserialize<'static> + Default + Clone {}

pub trait Description: Debug {
    /// If specified this should have the format "<type>/<org>/<category>/<name>",
    /// For example, "model/openai/chat/gpt-3.5-turbo"
    fn get_id(&self) -> String {
        String::new()
    }

    fn get_description(&self) -> String;
}
