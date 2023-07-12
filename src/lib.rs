#![allow(unused)]
use egui::Color32;

pub struct KolorWheel {
    count: u32,
    h: f32, s: f32, l: f32,
    r: u8, g: u8, b: u8,
}

impl KolorWheel {

    pub fn new() -> KolorWheel {
        Self {
            count: 1,
            h: 180.0, s: 0.0, l: 50.0,
            r: 127, g: 127, b: 127,
        }
    }

    pub fn set_count(mut self, count: u32) -> KolorWheel {
        self.count = count;
        return self;
    }

    pub fn set_rgb(mut self, r: u8, g: u8, b: u8) -> KolorWheel {

        self.r = r;
        self.g = g;
        self.b = b;

        return self;
    }


    pub fn set_color32(mut self, color: Color32) -> KolorWheel {
        return self.set_rgb(color.r(), color.g(), color.b());
    }

    fn validate_hsl(&mut self) {
        
        self.h = self.h % 360.0;

        if self.s < 0.0 { self.s = 0.0 };
        if self.s > 100.0 { self.s = 100.0 };

        if self.l < 0.0 { self.l = 0.0 };
        if self.l > 100.0 { self.l = 100.0 };
    }

    fn calc_rgb(mut self) {

		self.validate_hsl();
		
		let h = self.h / 360.0;
		let s = self.s / 100.0;
		let l = self.l / 100.0;

        if s < 0.001 {
            let lightness = (l * 255.0) as u8;
		    self.r = lightness;
		    self.g = lightness;
		    self.b = lightness;
            return;
        }

        let q = if l < 0.5 { 
            l * (1.0 + s)
        } else {
            l + s - l * s
        };
        let p = 2.0 * l - q;

        self.r = Self::hue_to_rgb(p, q, h + (1.0/3.0)) as u8;
	    self.g = Self::hue_to_rgb(p, q, h) as u8;
		self.b = Self::hue_to_rgb(p, q, h - (1.0/3.0)) as u8;
		
	} 

	fn hue_to_rgb(p: f32, q: f32, mut t: f32) -> f32{
		
		if t < 0.0 {
            t += 1.0;
        }
		if t > 1.0 {
            t -= 1.0;
        }
		if t < (1.0/6.0) {
            return p + (q - p) * 6.0 * t;
        }
		if t < (1.0/2.0) {
            return q;
        }
		if t < (2.0/3.0) {
            return p + (q - p) * ((2.0/3.0) - t) * 6.0;
        }
		
		return p;
	}

}

impl Iterator for KolorWheel {
    type Item = Color32;

    fn next(&mut self) -> Option<Color32>{

        if self.count == 0 {
            return None;
        }
        
        self.count -= 1;
        let color32 = Color32::DEBUG_COLOR;
        return Some(color32);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn does_nothing() {
        assert_eq!(2, 2);
    }
}
