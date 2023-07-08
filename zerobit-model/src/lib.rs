//! # Model
//!
//! Models are the core of the application. They provide access to multiple ppopular AI models that
//! can be used to generate text, image, etc.

mod error;
pub mod openai;
mod traits;

pub use error::ModelError;
pub use traits::Model;
