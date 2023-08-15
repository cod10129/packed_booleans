# packed_booleans
A crate for packing booleans together.

The standard `bool` type is always a full byte.
This becomes a problem when multiple booleans
are in the same struct, as 2 `bool`s are stored as 2 bytes,
which is unnecessary and wastes space.

This crate exists to solve that problem. 
The `PackedBools` type contains 8 booleans at the cost of only a single byte of memory.

## no_std
This crate is fully `no_std` compatible.
