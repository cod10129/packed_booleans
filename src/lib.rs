//! A crate for packing booleans together.

#![no_std]
#![warn(missing_docs)]

mod macros;
mod eight;
mod sixteen;

pub use eight::{PackedBools8, IntoIter8};
pub use sixteen::{PackedBools16};
