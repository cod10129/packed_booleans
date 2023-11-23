//! A crate for packing booleans together.

#![no_std]
#![warn(missing_docs)]

mod macros;
pub mod eight;

pub use eight::{PackedBools8, IntoIter8};
