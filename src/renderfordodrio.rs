//! Renders the QR code to different outputs.
//!
//! Outputs to a string representation and svg are supported.
use crate::matrix::Matrix;
use crate::qr::Qr;
//use crate::*;

/// A string renderer for converting a QR code into svg.
pub struct SvgDodrioRenderer {
    w: usize,
    h: usize,
    qz: bool,
}

impl SvgDodrioRenderer {
    /// Create a new renderer.
    pub fn new(w: usize, h: usize) -> Self {
        Self { w, h, qz: true }
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

        let mut res = String::from(format!(
            r##"<svg x="10%" y="22%" height="35%" width="80%" viewBox="0 0 {w} {h}"
            shape-rendering="crispEdges">
         <rect x="0" y="0" width="100%" height="100%" fill="#ffffff" />
         <path fill="#000000" d=" "##,
            w = self.w,
            h = self.h
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
