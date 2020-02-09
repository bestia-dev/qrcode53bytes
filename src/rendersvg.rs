//! Renders the QR code to different outputs.
//!
//! Outputs to a string representation and svg are supported.
use crate::matrix::{Matrix};
use crate::qr::Qr;
use crate::*;

/// A string renderer for converting a QR code into svg.
pub struct SvgRenderer {
    light: rendercommons::Color,
    dark: rendercommons::Color,
    w: usize,
    h: usize,
    qz: bool,
}

impl SvgRenderer {
    /// Create a new renderer.
    pub fn new() -> Self {
        Self {
            light: rendercommons::Color::new(255, 255, 255),
            dark: rendercommons::Color::new(0, 0, 0),
            w: 200,
            h: 200,
            qz: true,
        }
    }

    /// Set the light module color.
    /// Will also be the color of the quiet zone, if relevant.
    pub fn light_module(mut self, v: Color) -> Self {
        self.light = v;
        self
    }

    /// Set the dark module color.
    pub fn dark_module(mut self, v: Color) -> Self {
        self.dark = v;
        self
    }

    /// Set if quiet zone should be produced.
    pub fn quiet_zone(mut self, v: bool) -> Self {
        self.qz = v;
        self
    }

    /// Set the dimensions of the output, in pixels.
    /// Includes the quiet zone, if relevant.
    pub fn dimensions(mut self, w: usize, h: usize) -> Self {
        self.w = w;
        self.h = h;
        self
    }

    /// Render QR.
    pub fn render(&self, qr: &Qr) -> String {
        self.render_matrix(&qr.matrix)
    }

    /// Render matrix.
    pub fn render_matrix(&self, matrix: &Matrix) -> String {
        let cell_count = if self.qz {
            matrix.size + 8
        } else {
            matrix.size
        };
        // If not divided evenly adjust upwards and treat specified
        // width and height as minimums.
        let cell_w = ((self.w as f64) / (cell_count as f64)).ceil() as usize;
        let cell_h = ((self.h as f64) / (cell_count as f64)).ceil() as usize;
        // We might grow larger so readjust dimensions.
        let w = cell_w * cell_count;
        let h = cell_h * cell_count;
        
                let mut res = String::from(format!(
                    "<?xml version=\"1.0\" standalone=\"yes\"?>
        <svg xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\"
            viewBox=\"0 0 {w} {h}\" shape-rendering=\"crispEdges\">
        <rect x=\"0\" y=\"0\" width=\"{w}\" height=\"{h}\" fill=\"{light}\"/>
        <path fill=\"{dark}\" d=\"",
                    w = w,
                    h = h,
                    light = self.light.to_hex_str(),
                    dark = self.dark.to_hex_str()
                ));
        
        for y in 0..matrix.size {
            let yp = if self.qz {
                (y + 4) * cell_h
            } else {
                y * cell_h
            };

            for x in 0..matrix.size {
                let xp = if self.qz {
                    (x + 4) * cell_w
                } else {
                    x * cell_w
                };

                if matrix.is_dark(x, y) {
                    res.push_str(
                        format!(
                            "M{x} {y}h{w}v{h}H{x}V{y}",
                            x = xp,
                            y = yp,
                            w = cell_w,
                            h = cell_h
                        )
                        .as_str(),
                    );
                }
            }
        }
        res.push_str("\"/></svg>\n");
        res
    }
}
