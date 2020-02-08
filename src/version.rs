//! Specifies the version of a QR code.

use crate::mode::Mode;

/// QR code version, defines the size
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Version(pub usize);

impl Version {
    /// Create a new version, must be in the [1..40] range.
    pub fn new(v: usize) -> Version {
        assert!(v == 3);
        Version(v)
    }

    /// Return the data capacity. Only 3 L byte
    pub fn capacity(&self) -> usize {
        53
    }

    /// Return the size of the QR code.
    pub fn size(&self) -> usize {
        (((self.index()) * 4) + 21)
    }

    /// Returns the required len of the char count bit representation.
    pub fn char_count_len(&self, mode: Mode) -> usize {
        if self.0 >= 1 && self.0 <= 9 {
            match mode {
                Mode::Byte => 8,
            }
        } else {
            panic!("Malformed version {}", self.0);
        }
    }

    /// Returns the position of the dark module.
    pub fn dark_module_pos(&self) -> (usize, usize) {
        (8, 4 * self.0 + 9)
    }

    /// Return the version value - 1, suitable for indexing.
    pub fn index(&self) -> usize {
        (self.0 - 1)
    }
}
