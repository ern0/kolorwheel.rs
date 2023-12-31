use std::convert::From;

/// HSL representation of a color
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct HslColor {   
    /// - `h`: hue - degrees, 0..360, but it's okay to overflow/underflow
    pub h: f32, 
    /// - `s`: saturation - percent, 0..=100
    pub s: f32, 
    /// - `l`: lightness - percent, 0..=100
    pub l: f32,
}

impl From<(f32, f32, f32)> for HslColor {
    fn from((h, s, l): (f32, f32, f32)) -> HslColor {
        HslColor { h, s, l }
    }
}

impl From<(i32, i32, i32)> for HslColor {
    fn from((h, s, l): (i32, i32, i32)) -> HslColor {
        HslColor::new(h, s, l)
    }
}

impl From<[i32; 3]> for HslColor {
    fn from(hsl: [i32; 3]) -> HslColor {
        HslColor::new(hsl[0], hsl[1], hsl[2])
    }
}

impl From<&[i32; 3]> for HslColor {
    fn from(hsl: &[i32; 3]) -> HslColor {
        HslColor::new(hsl[0], hsl[1], hsl[2])
    }
}

impl HslColor {

    /// Constructor with integer values, for simplicity
    pub fn new(h: i32, s: i32, l: i32) -> Self {                
        Self {
            h: h as f32, 
            s: s as f32, 
            l: l as f32,
        }
    }

    pub(crate) fn normalize(&mut self) {
        
        self.h %= 360.0;
        if self.h < 0.0 { self.h += 360.0 };

        if self.s < 0.0 { self.s = 0.0 };
        if self.s > 100.0 { self.s = 100.0 };

        if self.l < 0.0 { self.l = 0.0 };
        if self.l > 100.0 { self.l = 100.0 };
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_float_eq::*;

    #[test]
    fn normalize_turnover() {
        let mut hsl = HslColor::new(370, 0, 0);
        hsl.normalize();
        assert_f32_near!(hsl.h, 10.0, 99999);

    }

    #[test]
    fn normalize_ceil_sat() {
        let mut hsl = HslColor::new(0, 120, 0);
        hsl.normalize();
        assert_f32_near!(hsl.s, 100.0, 99999);
    }

    #[test]
    fn normalize_floor_lit() {
        let mut hsl = HslColor::new(0, 0, -10);
        hsl.normalize();
        assert_f32_near!(hsl.l, 0.0, 99999);   
    }

}
