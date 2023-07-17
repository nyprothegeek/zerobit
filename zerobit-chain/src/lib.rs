//! # Chain
//!
//! Chains are an abstraction higher than zerobit Models.
//! Chains are suited for continous interaction with a model.
//! With Chains, there is a focus on the kind of processing that is done on the intermediate input and output of a model.
//!
//! For example, a Chain type could apply autoregression to the input of a model.

mod traits;

pub use traits::*;
