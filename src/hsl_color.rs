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

