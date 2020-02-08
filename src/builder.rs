//! QR code builder.
//!
//! Separated from the Qr struct to keep a simple and safe API.
//! But it's still possible to interface with the builder directly.
use crate::mask::Mask;
use crate::matrix::{Matrix, Module};
use crate::mode::Mode;
use crate::qr::Qr;
use crate::version::Version;
use crate::*;

use bitvec::*;

/// Builder for a QR code.
pub struct QrBuilder {
    /// Version to use.
    /// If not set a minimal version will be calculated.
    pub version: Version,
    /// Mask to use.
    /// If not set the optimal mask will be chosen per the QR specification.
    pub mask: Option<Mask>,
    /// Error correction level to use.
    /// If not set ECLevel::Q, which recovers 25% of data, will be used.
    pub ecl: ECLevel,
    /// Encoding mode to use.
    /// If not set will be inferred from input data.
    pub mode: Mode,

    /// Resulting matrix.
    ///
    /// Note that even though the matrix is not an Option it might still be invalid.
    /// This to simplify the implementation.
    pub matrix: Matrix,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Error {
    /// Mode doesn't support encoding the supplied message.
    UnsupportedMode,
    /// Message is too long for the supplied version, or larger than the max len.
    MessageTooLong,
    /// The builder was in an incomplete state when trying to create a QR.
    IncompleteBuilder,
}

impl QrBuilder {
    /// Create a new builder.
    pub fn new() -> QrBuilder {
        QrBuilder {
            version: Version::new(3),
            mask: None,
            ecl: ECLevel::L,
            mode: Mode::Byte,

            matrix: Matrix::new(0),
        }
    }

    /// Set mask to use. If not set the best mask will be chosen according to the QR spec.
    pub fn mask(mut self, mask: Mask) -> Self {
        self.mask = Some(mask);
        self
    }

    /// Set error correction. Will default to ECLevel::Q.
    pub fn ecl(mut self, ecl: ECLevel) -> Self {
        self.ecl = ecl;
        self
    }

    /// Build all elements and generate a QR code.
    pub fn into(mut self, s: &str) -> Result<Qr, Error> {
        self.add_all(s)?;
        self.into_qr()
    }

    /// Convert the builder into a QR code.
    pub fn into_qr(self) -> Result<Qr, Error> {
        if !self.complete() {
            return Err(Error::IncompleteBuilder);
        }

        Ok(Qr {
            matrix: self.matrix,

            version: self.version,
            ecl: self.ecl,
            mode: self.mode,
            mask: self.mask.unwrap(),
        })
    }

    /// Add all elements of a QR code.
    pub fn add_all(&mut self, s: &str) -> Result<(), Error> {
        self.ensure_settings()?;
        self.add_fun_patterns();
        self.add_data(s)?;
        self.mask_data();
        self.add_info();

        Ok(())
    }

    /// Add function patterns.
    pub fn add_fun_patterns(&mut self) {
        self.add_finders();
        self.add_alignments();
        self.add_timing_patterns();
        self.add_dark_module();
        self.add_reserved_areas();
    }

    /// Add data.
    pub fn add_data(&mut self, s: &str) -> Result<(), Error> {
        self.ensure_settings()?;

        let version = self.version;
        let mode = self.mode;

        let v = data::encode_with_mode(s, mode, version);
        let v = ec::add(v, self.version);
        self.add_raw_data(&v);

        Ok(())
    }

    /// Add raw data.
    pub fn add_raw_data(&mut self, v: &BitVec) {
        let mut vi = 0;
        for (x, y) in ZigZagIt::new(self.matrix.size) {
            if self.matrix.is_fun(x, y) {
                continue;
            }
            self.matrix.set_data(x, y, v[vi]);
            vi += 1;
        }
        assert_eq!(vi, v.len());
    }

    /// Mask data.
    pub fn mask_data(&mut self) {
        if let Some(mask) = self.mask {
            self.mask_with(mask);
        } else {
            self.mask_best();
        }
    }

    /// Mask by evaluating available masks and choose the best one.
    pub fn mask_best(&mut self) {
        let (mask, masked) = mask::mask(&self.matrix);
        self.mask = Some(mask);
        self.matrix = masked;
    }

    /// Mask using a specific mask.
    pub fn mask_with(&mut self, mask: Mask) {
        self.mask = Some(mask);
        self.matrix = mask::apply_mask(mask, &self.matrix);
    }

    /// Add info.
    pub fn add_info(&mut self) {
        self.add_format_info();
    }

    /// Add format info.
    pub fn add_format_info(&mut self) {
        // Hard assumption that we have necessary data.
        let format = info::format_info(self.mask.unwrap());
        self.add_format(&format);
    }

    /// Return true if the build is complete.
    fn complete(&self) -> bool {
        if self.mask.is_none() {
            return false;
        }
        if self.matrix.size == 0 {
            return false;
        }
        self.matrix.complete()
    }

    // Ensure we have required settings, otherwise decide from string.
    fn ensure_settings(&mut self) -> Result<(), Error> {
        // Ensure the matrix is initialized.
        if self.matrix.size == 0 {
            self.matrix = Matrix::new(self.version.size());
        }

        Ok(())
    }

    fn add_finders(&mut self) {
        let size = self.matrix.size;

        self.add_finder(0, 0);
        self.add_separator(0, 7, 7, 7);
        self.add_separator(7, 0, 7, 7);

        self.add_finder(size - 7, 0);
        self.add_separator(size - 8, 7, size - 1, 7);
        self.add_separator(size - 8, 0, size - 8, 7);

        self.add_finder(0, size - 7);
        self.add_separator(0, size - 8, 7, size - 8);
        self.add_separator(7, size - 8, 7, size - 1);
    }

    // x and y specifies the top left corner
    fn add_finder(&mut self, x: usize, y: usize) {
        self.matrix.set_square(x, y, 7, Module::Function(true));
        self.matrix
            .set_square_outline(x + 1, y + 1, 5, Module::Function(false));
    }

    fn add_separator(&mut self, x0: usize, y0: usize, x1: usize, y1: usize) {
        for a in x0..x1 + 1 {
            for b in y0..y1 + 1 {
                self.matrix.set(a, b, Module::Function(false));
            }
        }
    }

    fn add_alignments(&mut self) {
        // for byte-3-L
        let locations = &[6, 26];
        for x in locations.iter() {
            for y in locations.iter() {
                self.try_add_alignment(*x, *y);
            }
        }
    }

    // x and y specifies the center point
    fn try_add_alignment(&mut self, cx: usize, cy: usize) {
        let x = cx - 2;
        let y = cy - 2;
        if !self.matrix.any_in_square(x, y, 4) {
            self.matrix.set_square(x, y, 5, Module::Function(true));
            self.matrix
                .set_square_outline(x + 1, y + 1, 3, Module::Function(false));
        }
    }

    fn add_timing_patterns(&mut self) {
        let offset = 6;
        for i in offset..self.matrix.size - offset {
            let v = i % 2 == 0;
            self.set_timing(i, offset, v);
            self.set_timing(offset, i, v);
        }
    }

    fn set_timing(&mut self, x: usize, y: usize, v: bool) {
        // Timing patterns should always overlap with finders and alignment modules.
        if self.matrix.is_fun(x, y) {
            assert_eq!(self.matrix.is_dark(x, y), v, "timing overlap {},{}", x, y);
        }

        self.matrix.set(x, y, Module::Function(v));
    }

    fn add_dark_module(&mut self) {
        let (x, y) = self.version.dark_module_pos();
        self.matrix.set(x, y, Module::Function(true));
    }

    fn add_reserved_areas(&mut self) {
        let size = self.matrix.size;

        // Around top left finder.
        // Avoid timing pattern.
        self.reserve_rect(0, 8, 5, 8);
        self.reserve_rect(7, 8, 8, 8);
        self.reserve_rect(8, 0, 8, 5);
        self.reserve_rect(8, 7, 8, 7);

        // Top right.
        self.reserve_rect(size - 8, 8, size - 1, 8);

        // Bottom left.
        self.reserve_rect(8, size - 7, 8, size - 1);
    }

    fn reserve_rect(&mut self, x0: usize, y0: usize, x1: usize, y1: usize) {
        assert!(!self.matrix.any_in_rect(x0, y0, x1, y1));
        self.matrix.set_rect(x0, y0, x1, y1, Module::Reserved);
    }

    fn add_format(&mut self, bv: &BitVec) {
        assert_eq!(bv.len(), 15);
        let size = self.matrix.size;

        // Info surrounding the top left finder.
        let mut iter = bv.iter();
        for x in 0..8 {
            // Avoid timing pattern.
            if x == 6 {
                continue;
            }
            self.matrix.set_fun(x, 8, iter.next().unwrap());
        }
        for y in (0..9).rev() {
            // Avoid timing pattern.
            if y == 6 {
                continue;
            }
            self.matrix.set_fun(8, y, iter.next().unwrap());
        }
        assert_eq!(iter.next(), None);

        // Half to the right of the bottom left finder.
        iter = bv.iter();
        for y in (size - 7..size).rev() {
            self.matrix.set_fun(8, y, iter.next().unwrap());
        }
        // Rest bottom of the top left finder.
        for x in (size - 8)..size {
            self.matrix.set_fun(x, 8, iter.next().unwrap());
        }
        assert_eq!(iter.next(), None);
    }

    /// Convert to debug string.
    pub fn to_dbg_string(&self) -> String {
        render::to_dbg_string(&self.matrix)
    }
}

// A zig-zagging iterator which moves according to the QR data specification.
// It starts in the bottom right corner and moves flows in fields 2 bits wide
// up and down.
// Inside the 2 bit flow it alternates between the right and left field.
// It also avoids the vertical timing pattern column completely,
// but it does not automatically skip function patterns.
struct ZigZagIt {
    size: usize,
    // Should we move horizontal next step?
    horizontal_next: bool,
    // Are we moving upwards?
    upwards: bool,
    // xy coordinates into the matrix.
    x: usize,
    y: usize,
    // Valid? Used as a stop criteria.
    valid: bool,
}

impl ZigZagIt {
    fn new(size: usize) -> Self {
        Self {
            size: size,
            horizontal_next: true,
            upwards: true,
            x: size - 1,
            y: size - 1,
            valid: true,
        }
    }

    fn advance(&mut self) {
        if self.horizontal_next {
            self.move_horizontally();
        } else {
            self.move_vertically();
        }
    }

    fn move_horizontally(&mut self) {
        match self.x {
            0 => self.valid = false,
            6 => self.x -= 2,
            _ => self.x -= 1,
        }
        self.horizontal_next = false;
    }

    fn move_vertically(&mut self) {
        if (self.upwards && self.y == 0) || (!self.upwards && self.y == self.size - 1) {
            // When we've reached the edge move in the other direction instead of zagging.
            self.upwards = !self.upwards;
            self.move_horizontally();
        } else {
            // Zag motion, y is inverted
            if self.upwards {
                self.y -= 1;
            } else {
                self.y += 1;
            }
            self.x += 1;
        }
        self.horizontal_next = true;
    }
}

impl Iterator for ZigZagIt {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if !self.valid {
            return None;
        }

        let res = Some((self.x, self.y));
        self.advance();
        res
    }
}
