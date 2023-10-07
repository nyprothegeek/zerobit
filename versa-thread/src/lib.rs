//! # Thread
//!
//! Threads are an abstraction higher than versa Models.
//! They are suited for continous conversation with a model.
//! This type of interaction is usually referred to as `Chains` in other libraries.
//!
//! Threads, however, allow you to add custom middlewares that can transform the input and output of a model
//! and one of the common things to do with a thread is to collect conversation history and use it as input for the next prompt.

pub mod basic_thread;
mod error;
mod traits;

pub use error::*;
pub use traits::*;
