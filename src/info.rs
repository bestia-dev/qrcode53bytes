//! Contains various QR specific encoding info.

use crate::data;
use crate::mask::Mask;

use bitvec::prelude::*;

/// Returns the total codewords for a given version end error correction.
pub fn total_codewords() -> usize {
    let d = block_data();
    d.1 * d.2 + d.3 * d.4
}

/// Returns the total data bits possible for a given version and error correction.
pub fn total_bits() -> usize {
    8 * total_codewords()
}

/// Returns a vector of codewords counts per block.
/// The length specifies how many blocks there are and each element
/// how many codewords exist in that block.
pub fn group_block_count() -> Vec<usize> {
    let data = block_data();
    let mut v = Vec::new();
    v.extend((0..data.1).map(|_| data.2));
    v.extend((0..data.3).map(|_| data.4));
    v
}

/// Returns error correction codewords per block.
pub fn block_ec_count() -> usize {
    block_data().0
}

/// Returns the format BitVec representation to be embedded.
pub fn format_info(mask: Mask) -> BitVec<Lsb0 , u8> {
    let x = FORMAT_INFO[mask.0];
    let mut bv = BitVec::<Lsb0 , u8>::with_capacity(15);
    data::append(&mut bv, x as u32, 15);
    bv
}

fn block_data() -> (usize, usize, usize, usize, usize) {
    (15, 1, 55, 0, 0)
}

// Format information for mask x ECLevel.
static FORMAT_INFO: [u16; 8] = [
    0b111011111000100,
    0b111001011110011,
    0b111110110101010,
    0b111100010011101,
    0b110011000101111,
    0b110001100011000,
    0b110110001000001,
    0b110100101110110,
];
