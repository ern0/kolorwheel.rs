#![allow(unused)]
use std::convert::From;
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

impl RgbColor {

    pub fn new(r: u8, g: u8, b: u8) -> Self {                
        Self { r, g, b }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

/*    
    #[test]
    fn rgb_hex_long_unprefixed() {
        let kw = KolorWheel::new().set_rgb(0, 0, 0).set_rgb_hex("1af9cc");
        assert_eq!(kw.r, 0x1A);
        assert_eq!(kw.g, 0xF9);
        assert_eq!(kw.b, 0xCC);
    }

    #[test]
    fn rgb_hex_long_prefixed() {
        let kw = KolorWheel::new().set_rgb(0, 0, 0).set_rgb_hex("#d498ea");
        assert_eq!(kw.r, 0xD4);
        assert_eq!(kw.g, 0x98);
        assert_eq!(kw.b, 0xEA);
    }

    #[test]
    fn rgb_hex_short() {
        let kw = KolorWheel::new().set_rgb(0, 0, 0).set_rgb_hex("#C34");
        assert_eq!(kw.r, 0xCC);
        assert_eq!(kw.g, 0x33);
        assert_eq!(kw.b, 0x44);
    }

    #[test]
    fn rgb_hex_invalid_length() {
        let kw = KolorWheel::new().set_rgb(0, 0, 0).set_rgb_hex("#21");
        assert_eq!(kw.r, 0);
        assert_eq!(kw.g, 0);
        assert_eq!(kw.b, 0);
    }

    #[test]
    fn rgb_hex_invalid_digit() {
        let kw = KolorWheel::new().set_rgb(0, 0, 0).set_rgb_hex("12345G");
        assert_eq!(kw.r, 0);
        assert_eq!(kw.g, 0);
        assert_eq!(kw.b, 0);
    }

*/

}

