# A crate for packing booleans

[<img alt="crates.io" src="https://img.shields.io/crates/v/packed_booleans?style=flat-square&color=orange&logo=rust">](https://crates.io/crates/packed_booleans)
[<img src="https://img.shields.io/badge/docs.rs-packed__booleans-blue?style=flat-square&logo=docs.rs">](https://docs.rs/packed_booleans)

The standard `bool` type is always a full byte.
This becomes a problem when multiple booleans
are in the same struct, as 2 `bool`s are stored as 2 bytes,
which is unnecessary and wastes space.

This crate exists to solve that problem. 
The `PackedBools` type contains 8 booleans at the cost of only a single byte of memory.

## no_std
This crate is fully `no_std` compatible.
