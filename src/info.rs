//! Contains various QR specific encoding info.

use crate::data;
use crate::ec::ECLevel;
use crate::mask::Mask;

use bitvec::*;

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
pub fn format_info(ecl: ECLevel, mask: Mask) -> BitVec {
    let x = FORMAT_INFO[mask.0][ecl as usize];
    let mut bv = BitVec::with_capacity(15);
    data::append(&mut bv, x as u32, 15);
    bv
}

fn block_data() -> (usize, usize, usize, usize, usize) {
    (15, 1, 55, 0, 0)
}

// Format information for mask x ECLevel.
static FORMAT_INFO: [[u16; 4]; 8] = [
    [
        0b111011111000100,
        0b101010000010010,
        0b011010101011111,
        0b001011010001001,
    ],
    [
        0b111001011110011,
        0b101000100100101,
        0b011000001101000,
        0b001001110111110,
    ],
    [
        0b111110110101010,
        0b101111001111100,
        0b011111100110001,
        0b001110011100111,
    ],
    [
        0b111100010011101,
        0b101101101001011,
        0b011101000000110,
        0b001100111010000,
    ],
    [
        0b110011000101111,
        0b100010111111001,
        0b010010010110100,
        0b000011101100010,
    ],
    [
        0b110001100011000,
        0b100000011001110,
        0b010000110000011,
        0b000001001010101,
    ],
    [
        0b110110001000001,
        0b100111110010111,
        0b010111011011010,
        0b000110100001100,
    ],
    [
        0b110100101110110,
        0b100101010100000,
        0b010101111101101,
        0b000100000111011,
    ],
];
