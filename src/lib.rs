//! A crate for packing booleans together.

#![no_std]
#![warn(missing_docs)]

pub mod eight;

pub use eight::{PackedBools8, IntoIter8};

pub use PackedBools8 as PackedBools;
pub use IntoIter8 as IntoIter;
