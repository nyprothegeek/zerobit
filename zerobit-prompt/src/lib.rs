//! # Prompt
//!
//! This module contains implementation of the prompt templating feature.
//! This lets users create reusable prompts.

mod error;
mod macros;
mod prompt;
mod traits;

pub use error::*;
pub use prompt::*;
pub use traits::*;
