//! Renders the QR code to different outputs.
//!
//! Outputs to a string representation and svg are supported.
use crate::matrix::Matrix;
use crate::qr::Qr;

/// A string renderer for converting a QR code into a representation
/// suitable for text output.
pub struct StringRenderer {
    light: char,
    dark: char,
    module_w: usize,
    module_h: usize,
    qz: bool,
}

impl StringRenderer {
    /// Create a new renderer.
    pub fn new() -> Self {
        Self {
            light: '.',
            dark: '#',
            module_w: 1,
            module_h: 1,
            qz: false,
        }
    }

    /// Set the light module character.
    pub fn light_module(mut self, v: char) -> Self {
        self.light = v;
        self
    }

    /// Set the dark module character.
    pub fn dark_module(mut self, v: char) -> Self {
        self.dark = v;
        self
    }

    /// Set if quiet zone should be produced.
    pub fn quiet_zone(mut self, v: bool) -> Self {
        self.qz = v;
        self
    }

    /// Set the module dimensions, in character count per module.
    pub fn module_dimensions(mut self, w: usize, h: usize) -> Self {
        assert!(w > 0 && h > 0);
        self.module_w = w;
        self.module_h = h;
        self
    }

    /// Render QR to string.
    pub fn render(&self, qr: &Qr) -> String {
        self.render_matrix(&qr.matrix)
    }

    /// Render matrix to string.
    pub fn render_matrix(&self, matrix: &Matrix) -> String {
        let mut res = String::with_capacity(matrix.size * matrix.size);
        self.qz_lines(&mut res);
        for y in 0..matrix.size {
            // Duplicate rows for larger module dimensions.
            for _ in 0..self.module_h {
                let mut s = String::with_capacity(matrix.size + 1);
                self.qz_chars(&mut s);
                for x in 0..matrix.size {
                    let c = if matrix.is_dark(x, y) {
                        self.dark
                    } else {
                        self.light
                    };
                    // Duplicate chars for larger module dimensions.
                    for _ in 0..self.module_w {
                        s.push(c);
                    }
                }
                self.qz_chars(&mut s);
                s.push('\n');
                res.push_str(&s);
            }
        }
        self.qz_lines(&mut res);
        res
    }

    // Append empty lines for quiet zone padding.
    fn qz_lines(&self, s: &mut String) {
        if self.qz {
            for _ in 0..(4 * self.module_h) {
                s.push_str("\n");
            }
        }
    }

    // Append whitespace chars for quiet zone padding.
    fn qz_chars(&self, s: &mut String) {
        if self.qz {
            for _ in 0..(4 * self.module_w) {
                s.push(' ');
            }
        }
    }
}
