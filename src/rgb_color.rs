#![allow(unused)]
use std::convert::From;

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
    use assert_float_eq::*;

    #[test]
    fn rgb_black() {
        let mut rgb = RgbColor::from(&[0, 0, 0]);
    }
}
