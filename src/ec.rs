//! Error correction calculations.

use crate::info;
use crate::version::Version;

use bitvec::*;

/// Error correction level.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ECLevel {
  /// Recovers 7% of data
  L = 0,
}

impl ECLevel {
  /// Returns the bit encoding. It is not the same as the enum order.
  pub fn to_bitvec(&self) -> BitVec {
    match self {
      ECLevel::L => bitvec![0, 1],
    }
  }
}

/// Add error correction codewords to data.
///
/// This includes both the data and the error correction codewords,
/// interleaved if necessary.
pub fn add(data: BitVec, v: Version) -> BitVec {
  let layout = info::group_block_count();
  assert_eq!(data.len() / 8, layout.iter().sum());

  let blocks = group_into_blocks(&data, &layout);
  let mut bytes: Vec<u8> = Vec::with_capacity(data.len() / 8);

  // First interleave all codewords in blocks.
  let layout_max = layout.iter().max().unwrap();
  for i in 0..*layout_max {
    for block in blocks.iter() {
      if i < block.len() {
        bytes.push(block[i]);
      }
    }
  }

  // Then interleave all ec codewords in blocks.
  //for byte-3-L is 15
  let ec_count = 15;
  let ec_blocks: Vec<Vec<u8>> = blocks
    .iter()
    .map(|x| generate_ec_codewords(x.as_slice()))
    .collect();
  for i in 0..ec_count {
    for ec in ec_blocks.iter() {
      bytes.push(ec[i]);
    }
  }

  let mut res: BitVec = bytes.into();

  // Add padding remainder bits.
  let remainder = REMAINDER_BITS[v.index()];
  res.resize(res.len() + remainder, false);
  assert_eq!(
    res.len(),
    data.len() + 8 * ec_count * layout.len() + remainder
  );

  res
}

fn generate_ec_codewords(msg: &[u8]) -> Vec<u8> {
  //for byte-3-L is 15
  let ec_count = 15;
  let gen = GEN_POLYS;
  assert_eq!(gen.len(), ec_count);

  // res[i] corresponds to the constant before x^i.
  let mut res: Vec<u8> = msg.into();
  // Extending the vector effectively multiplies all constants with ec_count.
  res.resize(res.len() + ec_count, 0);

  for i in 0..msg.len() {
    let lead = res[i] as usize;
    // Term is zero, nothing to do.
    if lead == 0 {
      continue;
    }

    // Use alpha notation for multiplications.
    let alpha = LOG[lead] as usize;
    // For all remaining terms, xor with the current result.
    for (x, y) in res[i + 1..].iter_mut().zip(gen.iter()) {
      // All 2^n in GF(256) are precalculated.
      *x ^= EXP[((*y as usize) + alpha) % 255];
    }
  }
  // Last ec_count elements is our result.
  let v: Vec<u8> = res[msg.len()..].into();
  assert_eq!(v.len(), ec_count);
  v
}

fn group_into_blocks(bv: &BitVec, layout: &Vec<usize>) -> Vec<Vec<u8>> {
  let data = bv.as_slice();
  assert_eq!(data.len(), layout.iter().sum());

  let mut res = Vec::with_capacity(layout.len());
  let mut data_it = data.iter();
  for block in layout.iter() {
    let mut block_v: Vec<u8> = Vec::with_capacity(*block);
    for _ in 0..*block {
      block_v.push(*data_it.next().unwrap());
    }
    res.push(block_v);

    // Maybe there's a more idiomatic way to populate res using the take() method on data_it,
    // but I couldn't get past the borrow checker. I tried this:
    //res.push(data_it.take(*block).map(|x| *x as u8).collect());
  }
  res
}

// How many additional remainder bits needs to be added
// after interleaving blocks and ec codes?
// Only depends on the version.
static REMAINDER_BITS: [usize; 40] = [
  0, 7, 7, 7, 7, 7, 0, 0, 0, 0, 0, 0, 0, 3, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 3, 3, 3, 3, 3,
  3, 3, 0, 0, 0, 0, 0, 0,
];

// Encode 2^x in GF(256) arithmetic.
static EXP: [u8; 256] = [
  1, 2, 4, 8, 16, 32, 64, 128, 29, 58, 116, 232, 205, 135, 19, 38, 76, 152, 45, 90, 180, 117, 234,
  201, 143, 3, 6, 12, 24, 48, 96, 192, 157, 39, 78, 156, 37, 74, 148, 53, 106, 212, 181, 119, 238,
  193, 159, 35, 70, 140, 5, 10, 20, 40, 80, 160, 93, 186, 105, 210, 185, 111, 222, 161, 95, 190,
  97, 194, 153, 47, 94, 188, 101, 202, 137, 15, 30, 60, 120, 240, 253, 231, 211, 187, 107, 214,
  177, 127, 254, 225, 223, 163, 91, 182, 113, 226, 217, 175, 67, 134, 17, 34, 68, 136, 13, 26, 52,
  104, 208, 189, 103, 206, 129, 31, 62, 124, 248, 237, 199, 147, 59, 118, 236, 197, 151, 51, 102,
  204, 133, 23, 46, 92, 184, 109, 218, 169, 79, 158, 33, 66, 132, 21, 42, 84, 168, 77, 154, 41, 82,
  164, 85, 170, 73, 146, 57, 114, 228, 213, 183, 115, 230, 209, 191, 99, 198, 145, 63, 126, 252,
  229, 215, 179, 123, 246, 241, 255, 227, 219, 171, 75, 150, 49, 98, 196, 149, 55, 110, 220, 165,
  87, 174, 65, 130, 25, 50, 100, 200, 141, 7, 14, 28, 56, 112, 224, 221, 167, 83, 166, 81, 162, 89,
  178, 121, 242, 249, 239, 195, 155, 43, 86, 172, 69, 138, 9, 18, 36, 72, 144, 61, 122, 244, 245,
  247, 243, 251, 235, 203, 139, 11, 22, 44, 88, 176, 125, 250, 233, 207, 131, 27, 54, 108, 216,
  173, 71, 142, 1,
];

// Encode the inverse of EXP.
static LOG: [u8; 256] = [
  255, 0, 1, 25, 2, 50, 26, 198, 3, 223, 51, 238, 27, 104, 199, 75, 4, 100, 224, 14, 52, 141, 239,
  129, 28, 193, 105, 248, 200, 8, 76, 113, 5, 138, 101, 47, 225, 36, 15, 33, 53, 147, 142, 218,
  240, 18, 130, 69, 29, 181, 194, 125, 106, 39, 249, 185, 201, 154, 9, 120, 77, 228, 114, 166, 6,
  191, 139, 98, 102, 221, 48, 253, 226, 152, 37, 179, 16, 145, 34, 136, 54, 208, 148, 206, 143,
  150, 219, 189, 241, 210, 19, 92, 131, 56, 70, 64, 30, 66, 182, 163, 195, 72, 126, 110, 107, 58,
  40, 84, 250, 133, 186, 61, 202, 94, 155, 159, 10, 21, 121, 43, 78, 212, 229, 172, 115, 243, 167,
  87, 7, 112, 192, 247, 140, 128, 99, 13, 103, 74, 222, 237, 49, 197, 254, 24, 227, 165, 153, 119,
  38, 184, 180, 124, 17, 68, 146, 217, 35, 32, 137, 46, 55, 63, 209, 91, 149, 188, 207, 205, 144,
  135, 151, 178, 220, 252, 190, 97, 242, 86, 211, 171, 20, 42, 93, 158, 132, 60, 57, 83, 71, 109,
  65, 162, 31, 45, 67, 216, 183, 123, 164, 118, 196, 23, 73, 236, 127, 12, 111, 246, 108, 161, 59,
  82, 41, 157, 85, 170, 251, 96, 134, 177, 187, 204, 62, 90, 203, 89, 95, 176, 156, 169, 160, 81,
  11, 245, 22, 235, 122, 117, 44, 215, 79, 174, 213, 233, 230, 231, 173, 232, 116, 214, 244, 234,
  168, 80, 88, 175,
];

// Generator polynomials for the different grades.
// Just hardcode them, I can't be bothered to write the generator myself...
static GEN_POLYS: &[u8] = &[
  8, 183, 61, 91, 202, 37, 51, 58, 58, 237, 140, 124, 5, 99, 105,
];
