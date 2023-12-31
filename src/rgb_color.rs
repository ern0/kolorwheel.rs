use std::convert::From;
use std::str::FromStr;

/// RGB representation of a color, which
/// can be implicitly converted (`From/Into`) to [`RgbColor`]
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash, Debug)]
pub struct RgbColor {
    /// Red channel (0..=255)
    pub r: u8,  
    /// Green channel (0..=255)
    pub g: u8,  
    /// Blue channel (0..=255)
    pub b: u8,  
}

impl From<(u8, u8, u8)> for RgbColor {
    fn from((r, g, b): (u8, u8, u8)) -> RgbColor {
        RgbColor { r, g, b }
    }
}

impl From<[u8; 3]> for RgbColor {
    fn from(rgb: [u8; 3]) -> RgbColor {
        RgbColor { r: rgb[0], g: rgb[1], b: rgb[2] }
    }
}

impl From<&[u8; 3]> for RgbColor {
    fn from(rgb: &[u8; 3]) -> RgbColor {
        RgbColor { r: rgb[0], g: rgb[1], b: rgb[2] }
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

#[derive(Debug)]
/// Possible errors when parsing [`HslColor`](`crate::HslColor`) from hex `&str`
pub enum ParseError {
    /// Only `RRGGBB` or `RGB` variants are accepted, with optional "`#`" prefix.
    /// Reporting invalid length value
    InvalidLength(usize),
    /// Only hexadecimal digits can be used, any case.
    /// Reporting invalid character (first one)
    InvalidDigit(u8),
}

/// Create RGB color from hex `str`:
/// - "`#`" prefix is optional
/// - both `RGB` and `RRGGBB` format is accepted
impl FromStr for RgbColor {
    type Err = ParseError;

    fn from_str(hex: &str) -> Result<RgbColor, Self::Err> {
        Self::try_parse_hex_auto(hex)
    }
}

impl RgbColor {

    fn try_parse_hex_auto(hex: &str) -> Result<RgbColor, ParseError> {

        let mut hexb = hex.as_bytes();

        if hexb.is_empty() {
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

        Ok(RgbColor { r, g, b })
    }

    fn parse_hex_digit(digit: u8) -> Result<u8, ParseError> {

        if digit.is_ascii_digit() {
            return Ok(digit - b'0');
        }
        if (b'a'..=b'f').contains(&digit) {
            return Ok(10 + digit - b'a');
        }
        if (b'A'..=b'F').contains(&digit) {
            return Ok(10 + digit - b'A');
        }

        Err(ParseError::InvalidDigit(digit))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rgb_hex_long_unprefixed() {
        let rgb_result = RgbColor::from_str("1af9cC");
        assert!(matches!(rgb_result, Ok(_)));
        let rgb = rgb_result.unwrap();
        assert_eq!(rgb.r, 0x1A);
        assert_eq!(rgb.g, 0xF9);
        assert_eq!(rgb.b, 0xCC);
    }

    #[test]
    fn rgb_hex_long_prefixed() {
        let rgb_result = RgbColor::from_str("#d498ea");
        assert!(matches!(rgb_result, Ok(_)));
        let rgb = rgb_result.unwrap();
        assert_eq!(rgb.r, 0xD4);
        assert_eq!(rgb.g, 0x98);
        assert_eq!(rgb.b, 0xEA);
    }

    #[test]
    fn rgb_hex_short() {
        let rgb_result = RgbColor::from_str("#C34");
        assert!(matches!(rgb_result, Ok(_)));
        let rgb = rgb_result.unwrap();
        assert_eq!(rgb.r, 0xCC);
        assert_eq!(rgb.g, 0x33);
        assert_eq!(rgb.b, 0x44);
    }

    #[test]
    fn rgb_hex_invalid_length() {
        let rgb_result = RgbColor::from_str("#21");
        assert!(matches!(rgb_result, Err(ParseError::InvalidLength(2))));
    }

    #[test]
    fn rgb_hex_invalid_digit() {
        let rgb_result = RgbColor::from_str("12345G");
        assert!(matches!(rgb_result, Err(ParseError::InvalidDigit(b'G'))));
    }

    #[test]
    fn rgb_hex_invalid_empty() {
        let rgb_result = RgbColor::from_str("");
        assert!(matches!(rgb_result, Err(ParseError::InvalidLength(0))));
    }
}
