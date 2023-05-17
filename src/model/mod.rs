//! # Model
//!
//! Models are the core of the application. They provide access to multiple ppopular AI models that
//! can be used to generate text, image, etc.

mod anthropic;
mod cohere;
mod openai;

//-------------------------------------------------------------------------------------------------
// Traits
//-------------------------------------------------------------------------------------------------

/// A trait for models.
pub trait Model {}
