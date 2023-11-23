//! A crate for packing booleans together.

#![no_std]
#![warn(missing_docs)]

mod macros;
mod eight;

pub use eight::{PackedBools8, IntoIter8};
