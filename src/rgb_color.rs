#![allow(unused)]
use std::convert::{From, TryFrom};
use thiserror::Error;
use crate::hsl_color::HslColor;

#[derive(Clone, Copy, Default, PartialEq, Debug)]
pub struct RgbColor {
    pub r: u8, 
    pub g: u8, 
    pub b: u8,
}

impl From<(u8, u8, u8)> for RgbColor {
    fn from((r, g, b): (u8, u8, u8)) -> RgbColor {
        RgbColor::new(r, g, b)
    }
}

impl From<[u8; 3]> for RgbColor {
    fn from(rgb: [u8; 3]) -> RgbColor {
        RgbColor::new(rgb[0], rgb[1], rgb[2])
    }
}

impl From<&[u8; 3]> for RgbColor {
    fn from(rgb: &[u8; 3]) -> RgbColor {
        RgbColor::new(rgb[0], rgb[1], rgb[2])
    }
}

impl From<[f32; 3]> for RgbColor {
    fn from([r, g, b]: [f32; 3]) -> Self {
        let r = f32::round(r.clamp(0.0, 1.0) * 255.0) as u8;
        let g = f32::round(g.clamp(0.0, 1.0) * 255.0) as u8;
        let b = f32::round(b.clamp(0.0, 1.0) * 255.0) as u8;
        RgbColor { r, g, b }
    }
}

impl From<&[f32; 3]> for RgbColor {
    fn from([r, g, b]: &[f32; 3]) -> Self {
        let r = f32::round(r.clamp(0.0, 1.0) * 255.0) as u8;
        let g = f32::round(g.clamp(0.0, 1.0) * 255.0) as u8;
        let b = f32::round(b.clamp(0.0, 1.0) * 255.0) as u8;
        RgbColor { r, g, b }
    }
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("invalid length: {0}")]
    InvalidLength(usize),
    #[error("invalid digit: {0}")]
    InvalidDigit(u8),
}

impl TryFrom<&str> for RgbColor {
    type Error = ParseError;

    fn try_from(hex: &str) -> Result<RgbColor, Self::Error> {
        Self::try_parse_hex_auto(hex)
    }
}

impl RgbColor {

    fn try_parse_hex_auto(hex: &str) -> Result<RgbColor, ParseError> {

        let mut hexb = hex.as_bytes();

        if hexb.len() == 0 {
            return Err(ParseError::InvalidLength(0));
        }

        if hexb[0] == b'#' { hexb = &hexb[1..]; }
        let len = hexb.len();

        match len {
            3 => Self::parse_hex_6_digits(&[hexb[0], hexb[0], hexb[1], hexb[1], hexb[2], hexb[2]]),
            6 => Self::parse_hex_6_digits(hexb),
            _ => Err(ParseError::InvalidLength(len)),
        }
    }

    fn parse_hex_6_digits(hexb: &[u8]) -> Result<RgbColor, ParseError> {

        let r_hi = Self::parse_hex_digit(hexb[0])?;
        let r_lo = Self::parse_hex_digit(hexb[1])?;
        let g_hi = Self::parse_hex_digit(hexb[2])?;
        let g_lo = Self::parse_hex_digit(hexb[3])?;
        let b_hi = Self::parse_hex_digit(hexb[4])?;
        let b_lo = Self::parse_hex_digit(hexb[5])?;

        let r = (r_hi << 4) + r_lo;
        let g = (g_hi << 4) + g_lo;
        let b = (b_hi << 4) + b_lo;

        Ok(RgbColor::new(r, g, b))
    }

    fn parse_hex_digit(digit: u8) -> Result<u8, ParseError> {

        if digit >= b'0' && digit <= b'9' {
            return Ok(digit - b'0');
        }
        if digit >= b'a' && digit <= b'f' {
            return Ok(10 + digit - b'a');
        }
        if digit >= b'A' && digit <= b'F' {
            return Ok(10 + digit - b'A');
        }

        Err(ParseError::InvalidDigit(digit))
    }

}

impl RgbColor {
    pub fn new(r: u8, g: u8, b: u8) -> Self {                
        Self { r, g, b }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rgb_hex_long_unprefixed() {
        let rgb_result = RgbColor::try_from("1af9cc");
        assert!(matches!(rgb_result, Ok(_)));
        let rgb = rgb_result.unwrap();
        assert_eq!(rgb.r, 0x1A);
        assert_eq!(rgb.g, 0xF9);
        assert_eq!(rgb.b, 0xCC);
    }

    #[test]
    fn rgb_hex_long_prefixed() {
        let rgb_result = RgbColor::try_from("#d498ea");
        assert!(matches!(rgb_result, Ok(_)));
        let rgb = rgb_result.unwrap();
        assert_eq!(rgb.r, 0xD4);
        assert_eq!(rgb.g, 0x98);
        assert_eq!(rgb.b, 0xEA);
    }

    #[test]
    fn rgb_hex_short() {
        let rgb_result = RgbColor::try_from("#C34");
        assert!(matches!(rgb_result, Ok(_)));
        let rgb = rgb_result.unwrap();
        assert_eq!(rgb.r, 0xCC);
        assert_eq!(rgb.g, 0x33);
        assert_eq!(rgb.b, 0x44);
    }

    #[test]
    fn rgb_hex_invalid_length() {
        let rgb_result = RgbColor::try_from("#21");
        assert!(matches!(rgb_result, Err(ParseError::InvalidLength(2))));
    }

    #[test]
    fn rgb_hex_invalid_digit() {
        let rgb_result = RgbColor::try_from("12345G");
        assert!(matches!(rgb_result, Err(ParseError::InvalidDigit(b'G'))));
    }

    #[test]
    fn rgb_hex_invalid_empty() {
        let rgb_result = RgbColor::try_from("");
        assert!(matches!(rgb_result, Err(ParseError::InvalidLength(0))));
    }
}
