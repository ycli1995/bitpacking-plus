# Wrappers of bitpacking with extra pack formats

[![Crates.io](https://img.shields.io/crates/v/bitpacking-plus?color=green&style=for-the-badge)](https://crates.io/crates/bitpacking-plus)
[![docs.rs](https://img.shields.io/badge/docs.rs-latest-green?style=for-the-badge)](https://docs.rs/bitpacking-plus/)

This crate wraps crate `bitpacking`. It contains variant bitpacking formats, inspired by [BPCells](https://github.com/bnprks/BPCells).

See also this [article](https://bnprks.github.io/BPCells/articles/web-only/bitpacking-format.html)

## Bitpacking format

### The vanilla format
Same as behaviors in vanilla compression of [bitpacking](https://docs.rs/bitpacking/latest/bitpacking/trait.BitPacker.html#examples-without-delta-encoding).

### `m1` format
Same as behaviors in vanilla compression of [bitpacking](https://docs.rs/bitpacking/latest/bitpacking/trait.BitPacker.html#examples-without-delta-encoding), but with 1 subtracted from each value prior to compression.

### `d1` format
Same as behaviors in delta compression of [bitpacking](https://docs.rs/bitpacking/latest/bitpacking/trait.BitPacker.html#examples-with-delta-encoding), which transforms the original input into the difference between consecutive values prior to bitpacking. Therefore, the original input block must be sorted.

### `d1z` format
Similar to `d1` format but with zigzag encoding applied after difference encoding, where $zigzag(x) = 2x$ if $x > 0$, while $x < 0$, $zigzag(x) = -2x - 1$. This is best for lists of close but not fully sorted runs of integers.
