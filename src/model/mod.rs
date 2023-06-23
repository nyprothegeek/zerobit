//! # Model
//!
//! Models are the core of the application. They provide access to multiple ppopular AI models that
//! can be used to generate text, image, etc.

mod anthropic;
mod cohere;
mod openai;

pub use openai::*;

//-------------------------------------------------------------------------------------------------
// Traits
//-------------------------------------------------------------------------------------------------

/// A trait for models.
pub trait Model {
    /// The type of input the model takes.
    type Input;

    /// The type of output the model produces.
    type Output;

    /// Generates output from the given input.
    fn generate(&self, prompt: Self::Input) -> Self::Output;
}
