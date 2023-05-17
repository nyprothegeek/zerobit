#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_debug_implementations, missing_docs, rust_2018_idioms)]
#![deny(unreachable_pub, private_in_public)]

//! # zerobit
//!
//! zerobit is a crate for creating reusable prompts and working with large language models that can be made into agents
//! capable of using tools or operating autonomously.

pub mod agent;
pub mod memory;
pub mod model;
pub mod prompt;
pub mod tool;
pub mod workflow;
