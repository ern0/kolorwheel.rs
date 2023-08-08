#![allow(unused)]
use std::convert::{From, Into};
use crate::rgb_color::RgbColor;

#[derive(Clone, Copy, Default, PartialEq, Debug)]
pub struct HslColor {
    pub h: f32, 
    pub s: f32, 
    pub l: f32,
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


impl Into<RgbColor> for HslColor {
    fn into(self) -> RgbColor {
        self.convert_hsl_to_rgb()
    }
}

impl HslColor {

    fn convert_hsl_to_rgb(&self) -> RgbColor {
        
        let h = self.h / 360.0;
        let s = self.s / 100.0;
        let l = self.l / 100.0;

        if s < 0.001 {
            let gray = (l * 255.0) as u8;
            return RgbColor::new(gray, gray, gray);
        }

        let q = if l < 0.5 { 
            l * (1.0 + s)
        } else {
            l + s - (l * s)
        };
        let p = (2.0 * l) - q;

        let r = Self::hue_to_rgb_component(p, q, h + (1.0/3.0));
        let g = Self::hue_to_rgb_component(p, q, h);
        let b = Self::hue_to_rgb_component(p, q, h - (1.0/3.0));

        let r = (r * 12000.0).round() / 12000.0;
        let g = (g * 12000.0).round() / 12000.0;
        let b = (b * 12000.0).round() / 12000.0;

        let r = (r * 255.0) as u8;
        let g = (g * 255.0) as u8;
        let b = (b * 255.0) as u8;
        
        RgbColor::new(r, g, b)
    } 

    fn hue_to_rgb_component(p: f32, q: f32, mut t: f32) -> f32 {
        
        if t < 0.0 {
            t += 1.0;
        }
        if t > 1.0 {
            t -= 1.0;
        }
        if t < (1.0/6.0) {
            return p + ((q - p) * 6.0 * t);
        }
        if t < (1.0/2.0) {
            return q;
        }
        if t < (2.0/3.0) {
            return p + ((q - p) * ((2.0/3.0) - t) * 6.0);
        }
        
        p
    }
}

impl HslColor {

    pub fn new(h: i32, s: i32, l: i32) -> Self {                
        Self {
            h: h as f32, 
            s: s as f32, 
            l: l as f32,
        }
    }


    fn normalize_hsl(h: &mut f32, s: &mut f32, l: &mut f32) {
        
        *h = *h % 360.0;
        if *h < 0.0 { *h += 360.0 };

        if *s < 0.0 { *s = 0.0 };
        if *s > 100.0 { *s = 100.0 };

        if *l < 0.0 { *l = 0.0 };
        if *l > 100.0 { *l = 100.0 };
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_float_eq::*;

    #[test]
    fn hsl_into_rgb_black() {
        let mut hsl = HslColor::new(0, 0, 0);
        let rgb: RgbColor = hsl.into();
        assert_eq!(rgb, RgbColor::new(0, 0, 0));
    }

    #[test]
    fn hsl_from_rgb_black() {
        let mut hsl = HslColor::from(RgbColor::from([0, 0, 0]));
        assert_f32_near!(hsl.h, 0.0);
        assert_f32_near!(hsl.s, 0.0);
        assert_f32_near!(hsl.l, 0.0);
    }

/*
    #[test]
    fn hsl_to_rgb_white() {
        let kw = KolorWheel::new().set_hsl(0, 100, 100);
        assert_eq!((kw.r, kw.g, kw.b,), (255, 255, 255));     
    }

    #[test]
    fn hsl_to_rgb_gray() {
        let kw = KolorWheel::new().set_hsl(240, 0, 40);
        assert!(kw.r == kw.g);     
        assert!(kw.g == kw.b);     
        assert!(kw.r > 1);     
    }

    #[test]
    fn hsl_to_rgb_red() {
        let kw = KolorWheel::new().set_hsl(0, 100, 50);
        assert_eq!((kw.r, kw.g, kw.b,), (255, 0, 0));     
    }

    #[test]
    fn hsl_to_rgb_green() {
        let kw = KolorWheel::new().set_hsl(120, 100, 50);
        assert_eq!((kw.r, kw.g, kw.b,), (0, 255, 0));     
    }

    #[test]
    fn hsl_to_rgb_cyan() {
        let kw = KolorWheel::new().set_hsl(180, 100, 50);
        assert_eq!((kw.r, kw.g, kw.b,), (0, 255, 255));     
    }

    #[test]
    fn hsl_to_rgb_blue() {
        let kw = KolorWheel::new().set_hsl(240, 100, 50);
        assert_eq!((kw.r, kw.g, kw.b,), (0, 0, 255));     
    }

    #[test]
    fn hsl_to_rgb_overflow_cyan() {
        let kw = KolorWheel::new().set_hsl(360 + 180, 100, 50);
        assert_eq!((kw.r, kw.g, kw.b,), (0, 255, 255));     
    }

    #[test]
    fn hsl_to_rgb_light_blue() {
        let kw = KolorWheel::new().set_hsl(240, 100, 90);
        assert!(kw.r == kw.g);     
        assert!(kw.b > kw.r);
        assert!(kw.b > 240);
    }



    #[test]
    fn rgb_to_hsl_white() {
        let kw = KolorWheel::new().set_rgb(255, 255, 255);
        assert_f32_near!(kw.s, 0.0);
        assert_f32_near!(kw.l, 100.0);
    }

    #[test]
    fn rgb_to_hsl_gray_127() {
        let kw = KolorWheel::new().set_rgb(127, 127, 127);
        assert_f32_near!(kw.s, 0.0);
        assert_f32_near!(kw.l, 50.0, 99999);
    }

    #[test]
    fn rgb_to_hsl_gray_128() {
        let kw = KolorWheel::new().set_rgb(128, 128, 128);
        assert_f32_near!(kw.s, 0.0);
        assert_f32_near!(kw.l, 50.0, 99999);
    }

    #[test]
    fn rgb_to_hsl_light_red() {
        let kw = KolorWheel::new().set_rgb(255, 127, 127);
        assert_f32_near!(kw.h, 0.0, 99999);
        assert_f32_near!(kw.s, 100.0, 99999);
        assert_f32_near!(kw.l, 75.0, 99999);
    }

    #[test]
    fn rgb_to_hsl_deep_purple() {
        let kw = KolorWheel::new().set_rgb(80, 0, 120);
        assert_f32_near!(kw.h, 280.0, 99999);
        assert_f32_near!(kw.s, 100.0, 99999);
        assert_f32_near!(kw.l, 23.5, 99999);
    }

    #[test]
    fn rgb_to_hsl_deep_blue() {
        let kw = KolorWheel::new().set_rgb(0, 0, 31);
        assert_f32_near!(kw.h, 240.0, 99999);
        assert_f32_near!(kw.s, 100.0, 99999);
        assert_f32_near!(kw.l, 6.1, 99999);
    }

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
