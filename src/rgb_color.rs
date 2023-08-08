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

impl From<RgbColor> for HslColor {    
    fn from(RgbColor { r, g, b }: RgbColor) -> Self {

        let min = r.min(g).min(b);
        let max = r.max(g).max(b);
        let fmin = f32::from(min) / 255.0;
        let fmax = f32::from(max) / 255.0;
        let d = fmax - fmin;
        let l = (fmin + fmax) / 2.0;
        let s = if min == max {
            0.0
        } else {
            f32::clamp(d / (1.0 - f32::abs(2.0 * l - 1.0)), 0.0, 1.0)
        };
        let h = if min == max {
            0.0
        } else {
            let (r, g, b) = (f32::from(r), f32::from(g), f32::from(b));
            let numer = r - 0.5 * (g + b);
            let denom = f32::sqrt(r * r + g * g + b * b - r * g - r * b - g * b);
            let angle = f32::acos(numer / denom);

            if g < b { core::f32::consts::TAU - angle } else { angle }
        };
        
        HslColor {
            h: h.to_degrees(),
            s: s * 100.0,
            l: l * 100.0,
        }                    
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
