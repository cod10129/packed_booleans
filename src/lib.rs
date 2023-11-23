//! A crate for packing booleans together.

#![no_std]
#![warn(missing_docs)]

pub mod eight;

#[macro_use]
mod macros;

pub use eight::{PackedBools8, IntoIter8};
