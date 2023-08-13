/*! # Extra [bitpacking](https://docs.rs/bitpacking/latest/bitpacking/) formats

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

*/

#[cfg(test)]
#[macro_use]
pub(crate) mod tests;

pub mod convert;
pub use crate::convert::*;

use bitpacking::{BitPacker, BitPacker1x, BitPacker4x, BitPacker8x};

/** # Wrappers of the orignial bitpacking trait
The `panic` rules of `compressed` and `decompressed` array data are exactly the same as `bitpacking` crate. Details can
be found in [bitpacking::BitPacker](https://docs.rs/bitpacking/latest/bitpacking/trait.BitPacker.html).

## Note:
* The `num_bits` parameter in `pack*` methods can be set to 0, which means to detect actual `num_bits` by internally call
[`bitpacker.num_bits()`](https://docs.rs/bitpacking/latest/bitpacking/trait.BitPacker.html#tymethod.num_bits) or
[`bitpacker.num_bits_sorted()`](https://docs.rs/bitpacking/latest/bitpacking/trait.BitPacker.html#tymethod.num_bits_sorted)

## Examples
Here are some examples for the variant bitpacking formats (from tests.rs):

```
use bitpacking_plus::BitPackOps;

use bitpacking::{BitPacker, BitPacker1x, BitPacker4x, BitPacker8x};
use rand::{thread_rng, Rng};

#[derive(Debug)]
enum PackMethod {
    Vanilla,
    M1,
    D1,
    D1Z,
}

fn test_unpack_helper(
    bitpacker: &dyn BitPackOps,
    decompressed: &[u32],
    compressed: &mut [u8],
    block_size: usize,
    pack_method: PackMethod,
) {
    // We only test one block.
    println!("Test method: {:?}\nBlock size: {}", &pack_method, &block_size);
    let initial = decompressed[0];
    let n1 = match pack_method {
        PackMethod::Vanilla => {
            bitpacker.pack(decompressed.get(0..block_size).unwrap(), compressed, 0)
        }
        PackMethod::M1 => {
            bitpacker.pack_m1(decompressed.get(0..block_size).unwrap(), compressed, 0)
        }
        PackMethod::D1 => {
            bitpacker.pack_d1(decompressed.get(0..block_size).unwrap(), compressed, 0)
        }
        PackMethod::D1Z => {
            bitpacker.pack_d1z(decompressed.get(0..block_size).unwrap(), compressed, 0)
        }
    };
    let num_bits = 8 * n1 / block_size;
    let mut new_decompressed = [0_u32; 256];
    let n2 = match pack_method {
        PackMethod::Vanilla => bitpacker.unpack(compressed, &mut new_decompressed, num_bits as u8),
        PackMethod::M1 => bitpacker.unpack_m1(compressed, &mut new_decompressed, num_bits as u8),
        PackMethod::D1 => {
            bitpacker.unpack_d1(initial, compressed, &mut new_decompressed, num_bits as u8)
        }
        PackMethod::D1Z => {
            bitpacker.unpack_d1z(initial, compressed, &mut new_decompressed, num_bits as u8)
        }
    };
    assert_eq!(n1, n2);
    assert_eq!(
        decompressed.get(0..block_size).unwrap(),
        new_decompressed.get(0..block_size).unwrap()
    );
    println!("Bytes used: {}", n1);
    println!(
        "Decompresed: {:?}\n",
        new_decompressed.get(0..block_size).unwrap()
    );
}

fn main() {
    let mut my_data: [u32; 256] = [(); 256].map(|_| thread_rng().gen_range(0..20000));
    println!("Orignial: {:?}\n", my_data);

    let mut compressed = [0_u8; 8192];

    let bitpacker4 = BitPacker4x::new();

    test_unpack_helper(&bitpacker4, &my_data, &mut compressed, BitPacker4x::BLOCK_LEN, PackMethod::Vanilla);
    test_unpack_helper(&bitpacker4, &my_data, &mut compressed, BitPacker4x::BLOCK_LEN, PackMethod::M1);
    test_unpack_helper(&bitpacker4, &my_data, &mut compressed, BitPacker4x::BLOCK_LEN, PackMethod::D1Z);

    // For `(un)pack_d1`, the decompressed data must be sorted.
    my_data.sort();

    test_unpack_helper(&bitpacker4, &my_data, &mut compressed, BitPacker4x::BLOCK_LEN, PackMethod::D1);
}
```
*/
pub trait BitPackOps {
    fn pack(&self, decompressed: &[u32], compressed: &mut [u8], num_bits: u8) -> usize;

    fn unpack(&self, compressed: &[u8], decompressed: &mut [u32], num_bits: u8) -> usize;

    fn pack_m1(&self, decompressed: &[u32], compressed: &mut [u8], num_bits: u8) -> usize;

    fn unpack_m1(&self, compressed: &[u8], decompressed: &mut [u32], num_bits: u8) -> usize;

    fn pack_d1(&self, decompressed: &[u32], compressed: &mut [u8], num_bits: u8) -> usize;

    fn unpack_d1(
        &self,
        initial: u32,
        compressed: &[u8],
        decompressed: &mut [u32],
        num_bits: u8,
    ) -> usize;

    fn pack_d1z(&self, decompressed: &[u32], compressed: &mut [u8], num_bits: u8) -> usize;

    fn unpack_d1z(
        &self,
        initial: u32,
        compressed: &[u8],
        decompressed: &mut [u32],
        num_bits: u8,
    ) -> usize;
}

macro_rules! impl_bit_pack_ops {
    ($bitpacker:ident) => {
        impl BitPackOps for $bitpacker {
            fn pack(&self, decompressed: &[u32], compressed: &mut [u8], num_bits: u8) -> usize {
                let num_bits: u8 = match num_bits {
                    0 => self.num_bits(decompressed),
                    _ => num_bits,
                };
                self.compress(decompressed, compressed, num_bits)
            }

            fn unpack(&self, compressed: &[u8], decompressed: &mut [u32], num_bits: u8) -> usize {
                self.decompress(compressed, decompressed, num_bits)
            }

            fn pack_m1(&self, decompressed: &[u32], compressed: &mut [u8], num_bits: u8) -> usize {
                let mut tmp = vec![0_u32; decompressed.len()];
                vanilla_to_m1(decompressed, tmp.as_mut_slice());
                self.pack(tmp.as_slice(), compressed, num_bits)
            }

            fn unpack_m1(
                &self,
                compressed: &[u8],
                decompressed: &mut [u32],
                num_bits: u8,
            ) -> usize {
                let n = self.unpack(compressed, decompressed, num_bits);
                let n_decompressed = n * 8 / num_bits as usize;
                m1_to_vanilla_self(decompressed.get_mut(0..n_decompressed).unwrap());
                return n;
            }

            fn pack_d1(&self, decompressed: &[u32], compressed: &mut [u8], num_bits: u8) -> usize {
                let initial = decompressed[0];
                let num_bits: u8 = match num_bits {
                    0 => self.num_bits_sorted(initial, decompressed),
                    _ => num_bits,
                };
                self.compress_sorted(initial, decompressed, compressed, num_bits)
            }

            fn unpack_d1(
                &self,
                initial: u32,
                compressed: &[u8],
                decompressed: &mut [u32],
                num_bits: u8,
            ) -> usize {
                self.decompress_sorted(initial, compressed, decompressed, num_bits)
            }

            fn pack_d1z(&self, decompressed: &[u32], compressed: &mut [u8], num_bits: u8) -> usize {
                let mut tmp = vec![0_u32; decompressed.len()];
                vanilla_to_d1z(decompressed, tmp.as_mut_slice());
                self.pack(tmp.as_slice(), compressed, num_bits)
            }

            fn unpack_d1z(
                &self,
                initial: u32,
                compressed: &[u8],
                decompressed: &mut [u32],
                num_bits: u8,
            ) -> usize {
                let n = self.decompress(compressed, decompressed, num_bits);
                d1z_to_vanilla_self(decompressed, initial);
                return n;
            }
        }
    };
}

impl_bit_pack_ops!(BitPacker1x);
impl_bit_pack_ops!(BitPacker4x);
impl_bit_pack_ops!(BitPacker8x);
