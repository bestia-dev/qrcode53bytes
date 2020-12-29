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

    /// Return the data capacity.
    pub fn capacity(&self) -> usize {
        53
    }

    /// Return the size of the QR code.
    pub fn size(&self) -> usize {
        ((self.index()) * 4) + 21
    }

    /// Returns the required len of the char count bit representation.
    pub fn char_count_len(&self, mode: Mode) -> usize {
        if self.0 == 3 {
            match mode {
                Mode::Byte => 8,
            }
        } else {
            panic!("Malformed version {}", self.0);
        }
    }

    /// Returns true if this version requires extra version areas.
    pub fn extra_version_areas(&self) -> bool {
        self.0 >= 7
    }

    /// Returns the position of the dark module.
    pub fn dark_module_pos(&self) -> (usize, usize) {
        (8, 4 * self.0 + 9)
    }

    /// Return the version value - 1, suitable for indexing.
    pub fn index(&self) -> usize {
        self.0 - 1
    }
}
