//! Provides a simple and safe API.

use crate::builder::*;
use crate::ec::ECLevel;
use crate::mask::Mask;
use crate::matrix::Matrix;
use crate::mode::Mode;
use crate::version::Version;

/// The QR code.
///
/// Encapsulates a matrix, the 2D-grid containing the QR modules
/// and some information about the QR code.
#[derive(Debug, PartialEq, Eq)]
pub struct Qr {
    /// Version of the QR code.
    pub version: Version,

    /// Error correction level.
    pub ecl: ECLevel,

    /// Encoding mode.
    pub mode: Mode,

    /// The modules.
    pub matrix: Matrix,

    /// The applied mask, 0 to 7.
    pub mask: Mask,
}

impl Qr {
    /// Create a new QR from a string.
    pub fn new(s: &str) -> Result<Qr, Error> {
        QrBuilder::new()
            .version(Version::new(3))
            .ecl(ECLevel::L)
            .into(s)
    }

    /// Returns the size of the QR code.
    pub fn size(&self) -> usize {
        self.version.size()
    }
}
