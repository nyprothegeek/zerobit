//! # Agent
//!
//! Agents are an abstraction over low level language models. For example, some agents are capable of long term memory and can interact with the outside world by using provided tools.
//! Agents can also operate autonomously, for example, by using a trigger to decide when to generate text.

mod simple_agent;
mod tool_agent;

//-------------------------------------------------------------------------------------------------
// Traits
//-------------------------------------------------------------------------------------------------

/// A trait for agents.
pub trait Agent {}
