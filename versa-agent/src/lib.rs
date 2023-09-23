//! # Agent
//!
//! Agents are a few abstractions higher than low level language models and chains.
//! Agents not only have automatic autoregression for long conversations, but they are also able decisions based on these interactions.
//! Agents are long-running processes and they can be improved with long term memory and tool use but not all agents support these.

mod agent;
mod permissions;

pub use agent::*;

//-------------------------------------------------------------------------------------------------
// Traits
//-------------------------------------------------------------------------------------------------

/// A trait for agents.
pub trait Agent {}
