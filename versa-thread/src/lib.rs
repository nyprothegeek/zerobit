//! # Thread
//!
//! Threads are an abstraction higher than versa Models.
//! Threads are suited for continous interaction with a model.
//! With Threads, there is a focus on the kind of processing that is done on the intermediate input and output of a model.
//!
//! For example, a Thread type could apply autoregression to the input of a model.

mod error;
pub mod simple_thread;
mod traits;

pub use error::*;
pub use traits::*;
