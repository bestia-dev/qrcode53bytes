//! Renders the QR code to different outputs.
//!
//! Outputs to a string representation and svg are supported.
use crate::matrix::{Matrix, Module};

use std::num::ParseIntError;
use std::str::FromStr;
use std::u8;

/// Convert to string, with chars for the different underlying representations.
pub fn to_dbg_string(matrix: &Matrix) -> String {
    let mut res = String::with_capacity(matrix.size * matrix.size);
    res.push('\n');
    for y in 0..matrix.size {
        let mut s = String::with_capacity(matrix.size + 1);
        for x in 0..matrix.size {
            let c = match matrix.get(x, y) {
                Module::Unknown => '?',
                Module::Reserved => '*',
                Module::Function(true) => '#',
                Module::Function(false) => '.',
                Module::Data(true) => 'X',
                Module::Data(false) => '-',
            };
            s.push(c);
        }
        s.push('\n');
        res.push_str(&s);
    }
    res
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
/// An RGB color implementation.
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    /// Create a new from rgb parts.
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Create a new color from a hex input.
    /// ```
    /// # use rqr::*;
    /// let c = Color::hex(0xff3214);
    /// ```
    pub fn hex(v: u32) -> Self {
        Self {
            r: (v >> 16) as u8,
            g: (v >> 8) as u8,
            b: v as u8,
        }
    }

    /// Create a new color from a length 4 input hex.
    /// ```
    /// # use rqr::*;
    /// // Short for "#770000"
    /// let c = Color::from_4_hex("#700");
    /// ```
    pub fn from_4_hex(s: &str) -> Result<Self, ParseColorError> {
        let chars: Vec<char> = s.chars().collect();
        if chars[0] != '#' {
            return Err(ParseColorError);
        }
        let r = u8::from_str_radix(&chars[1].to_string(), 16)?;
        let g = u8::from_str_radix(&chars[2].to_string(), 16)?;
        let b = u8::from_str_radix(&chars[3].to_string(), 16)?;
        Ok(Color {
            r: (r << 4) | r,
            g: (g << 4) | g,
            b: (b << 4) | b,
        })
    }

    /// Create a new color from a length 7 input hex.
    /// ```
    /// # use rqr::*;
    /// let c = Color::from_7_hex("#3477ff");
    /// ```
    pub fn from_7_hex(s: &str) -> Result<Self, ParseColorError> {
        if s[0..1] != *"#" {
            return Err(ParseColorError);
        }
        let r = u8::from_str_radix(&s[1..3], 16)?;
        let g = u8::from_str_radix(&s[3..5], 16)?;
        let b = u8::from_str_radix(&s[5..7], 16)?;
        Ok(Color { r, g, b })
    }

    /// Convert to a hex string.
    /// ```
    /// # use rqr::*;
    /// assert_eq!(Color::hex(0xff7312).to_hex_str(), "#ff7312");
    /// ```
    pub fn to_hex_str(&self) -> String {
        String::from(format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b))
    }
}

#[derive(Debug, Copy, Clone)]
/// An error from trying to parse a Color instance from string.
pub struct ParseColorError;

impl FromStr for Color {
    type Err = ParseColorError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.len() {
            4 => Color::from_4_hex(s),
            7 => Color::from_7_hex(s),
            _ => Err(ParseColorError),
        }
    }
}

impl From<ParseIntError> for ParseColorError {
    fn from(_: ParseIntError) -> Self {
        ParseColorError
    }
}
