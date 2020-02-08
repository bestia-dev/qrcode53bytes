//! Data encoding.

use crate::mode::Mode;
use crate::version::Version;
use crate::*;

use bitvec::*;
use std::cmp;

/// Encode string data to BitVec in a specific mode.
/// Does not include error correction codes, it only encodes the data portion.
pub fn encode_with_mode(s: &str, mode: Mode, version: Version) -> BitVec {
    let total_capacity = info::total_bits();

    // Encoding is mode, char count, data.
    let mut bv = mode.to_bitvec();
    bv.reserve(total_capacity);
    bv.append(&mut bitvec_char_count(s.len(), mode, version));
    bv.append(&mut bitvec_data(s, mode));
    assert!(bv.len() <= total_capacity);

    // Add up to 4 zero bits if we're below capacity.
    let zero_bits = cmp::min(total_capacity - bv.len(), 4);
    append(&mut bv, 0, zero_bits);
    assert!(bv.len() <= total_capacity);

    // If we're still below capacity add zero bits until we have full bytes.
    let zero_bits = (total_capacity - bv.len()) % 8;
    append(&mut bv, 0, zero_bits);
    assert!(bv.len() % 8 == 0);

    // Until we reach our capacity add pad bytes.
    for pad in [0xEC, 0x11].iter().cycle() {
        if bv.len() >= total_capacity {
            break;
        }
        append(&mut bv, *pad, 8);
    }
    assert_eq!(bv.len(), total_capacity);

    bv
}

/// Append data to bitvec of a certain len.
pub fn append(bv: &mut BitVec, v: u32, len: usize) {
    bv.extend((0..len).rev().map(|i| (v >> i) & 1 != 0));
}

fn bitvec_char_count(len: usize, mode: Mode, v: Version) -> BitVec {
    let mut bv = BitVec::new();
    append(&mut bv, len as u32, v.char_count_len(mode));
    bv
}

fn bitvec_data(s: &str, mode: Mode) -> BitVec {
    let bytes = string_to_bytes(s, mode);

    match mode {
        Mode::Byte => encode_byte_data(&bytes),
    }
}

fn encode_byte_data(v: &Vec<u8>) -> BitVec {
    // It's already in ISO 8859-1, or UTF-8
    v[..].into()
}

// Converts string to byte representation.
// Numeric and alphanumeric are compacted more.
fn string_to_bytes(s: &str, mode: Mode) -> Vec<u8> {
    match mode {
        Mode::Byte => s.bytes().collect(),
    }
}
