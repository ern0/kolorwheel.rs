use std::convert::From;
use crate::hsl_color::HslColor;
use crate::rgb_color::RgbColor;

impl From<HslColor> for RgbColor {
    fn from(val: HslColor) -> Self {
        val.convert_hsl_to_rgb()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hsl_to_rgb_black() {
        let hsl = HslColor::new(0, 0, 0);
        let rgb: RgbColor = hsl.into();
        assert_eq!(rgb, RgbColor::new(0, 0, 0));
    }

    #[test]
    fn hsl_to_rgb_white() {
        let hsl = HslColor::new(0, 100, 100);
        let rgb: RgbColor = hsl.into();
        assert_eq!(rgb, RgbColor::new(255, 255, 255));
    }

    #[test]
    fn hsl_to_rgb_gray() {
        let hsl = HslColor::new(240, 0, 40);
        let rgb: RgbColor = hsl.into();
        assert!(rgb.r == rgb.g);
        assert!(rgb.g == rgb.b);
        assert!(rgb.r > 1);
    }

    #[test]
    fn hsl_to_rgb_red() {
        let hsl = HslColor::new(0, 100, 50);
        let rgb: RgbColor = hsl.into();
        assert_eq!(rgb, RgbColor::new(255, 0, 0));
    }

    #[test]
    fn hsl_to_rgb_green() {
        let hsl = HslColor::new(120, 100, 50);
        let rgb: RgbColor = hsl.into();
        assert_eq!(rgb, RgbColor::new(0, 255, 0));
    }

    #[test]
    fn hsl_to_rgb_cyan() {
        let hsl = HslColor::new(180, 100, 50);
        let rgb: RgbColor = hsl.into();
        assert_eq!(rgb, RgbColor::new(0, 255, 255));
    }

    #[test]
    fn hsl_to_rgb_overflow_cyan() {
        let hsl = HslColor::new(360 + 180, 100, 50);
        let rgb: RgbColor = hsl.into();
        assert_eq!(rgb, RgbColor::new(0, 255, 255));
    }

    #[test]
    fn hsl_to_rgb_blue() {
        let hsl = HslColor::new(240, 100, 50);
        let rgb: RgbColor = hsl.into();
        assert_eq!(rgb, RgbColor::new(0, 0, 255));
    }

    #[test]
    fn hsl_to_rgb_light_blue() {
        let hsl = HslColor::new(240, 100, 90);
        let rgb: RgbColor = hsl.into();
        assert!(rgb.r == rgb.g);
        assert!(rgb.b > rgb.r);
        assert!(rgb.b > 240);
    }

}
