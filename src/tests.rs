use super::BitPackOps;

use bitpacking::{BitPacker, BitPacker1x, BitPacker8x};
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
    println!("Test method: {:?}", &pack_method);
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

#[test]
fn test_all() {
    let mut my_data: [u32; 256] = [(); 256].map(|_| thread_rng().gen_range(0..20000));
    println!("Orignial: {:?}\n", my_data);

    let mut compressed = [0_u8; 8192];

    let bitpacker1 = BitPacker1x::new();
    let bitpacker8 = BitPacker8x::new();

    test_unpack_helper(
        &bitpacker1,
        &my_data,
        &mut compressed,
        BitPacker1x::BLOCK_LEN,
        PackMethod::Vanilla,
    );
    test_unpack_helper(
        &bitpacker8,
        &my_data,
        &mut compressed,
        BitPacker8x::BLOCK_LEN,
        PackMethod::Vanilla,
    );

    test_unpack_helper(
        &bitpacker1,
        &my_data,
        &mut compressed,
        BitPacker1x::BLOCK_LEN,
        PackMethod::M1,
    );
    test_unpack_helper(
        &bitpacker8,
        &my_data,
        &mut compressed,
        BitPacker8x::BLOCK_LEN,
        PackMethod::M1,
    );

    my_data.sort();

    test_unpack_helper(
        &bitpacker1,
        &my_data,
        &mut compressed,
        BitPacker1x::BLOCK_LEN,
        PackMethod::D1,
    );
    test_unpack_helper(
        &bitpacker8,
        &my_data,
        &mut compressed,
        BitPacker8x::BLOCK_LEN,
        PackMethod::D1,
    );

    test_unpack_helper(
        &bitpacker1,
        &my_data,
        &mut compressed,
        BitPacker1x::BLOCK_LEN,
        PackMethod::D1Z,
    );
    test_unpack_helper(
        &bitpacker8,
        &my_data,
        &mut compressed,
        BitPacker8x::BLOCK_LEN,
        PackMethod::D1Z,
    );
}
