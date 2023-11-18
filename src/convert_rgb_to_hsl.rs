use std::convert::From;
use crate::hsl_color::HslColor;
use crate::rgb_color::RgbColor;

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

#[cfg(test)]
mod tests {
    use super::*;
    use assert_float_eq::*;

    #[test]
    fn rgb_to_hsl_white() {
        let rgb = RgbColor { r: 255, g: 255, b: 255 };
        let hsl = HslColor::from(rgb);
        assert_f32_near!(hsl.s, 0.0);
        assert_f32_near!(hsl.l, 100.0);
    }


    #[test]
    fn rgb_to_hsl_gray_127() {
        let rgb = RgbColor { r: 127, g: 127, b: 127 };
        let hsl = HslColor::from(rgb);
        assert_f32_near!(hsl.s, 0.0);
        assert_f32_near!(hsl.l, 50.0, 99999);
    }

    #[test]
    fn rgb_to_hsl_gray_128() {
        let rgb = RgbColor { r: 128, g: 128, b: 128 };
        let hsl = HslColor::from(rgb);
        assert_f32_near!(hsl.s, 0.0);
        assert_f32_near!(hsl.l, 50.0, 99999);
    }

    #[test]
    fn rgb_to_hsl_light_red() {
        let rgb = RgbColor { r: 255, g: 127, b: 127 };
        let hsl = HslColor::from(rgb);
        assert_f32_near!(hsl.h, 0.0, 99999);
        assert_f32_near!(hsl.s, 100.0, 99999);
        assert_f32_near!(hsl.l, 75.0, 99999);
    }

    #[test]
    fn rgb_to_hsl_deep_purple() {
        let rgb = RgbColor { r: 80, g: 0, b: 120 };
        let hsl = HslColor::from(rgb);
        assert_f32_near!(hsl.h, 280.0, 99999);
        assert_f32_near!(hsl.s, 100.0, 99999);
        assert_f32_near!(hsl.l, 23.5, 99999);
    }

    #[test]
    fn rgb_to_hsl_deep_blue() {
        let rgb = RgbColor { r: 0, g: 0, b: 31 };
        let hsl = HslColor::from(rgb);
        assert_f32_near!(hsl.h, 240.0, 99999);
        assert_f32_near!(hsl.s, 100.0, 99999);
        assert_f32_near!(hsl.l, 6.1, 99999);
    }

}
