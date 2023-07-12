#![allow(unused)]
use egui::Color32;

pub struct KolorWheel {
    count: u32,
    h: f32, s: f32, l: f32,
    r: u8, g: u8, b: u8,
}

impl KolorWheel {

    pub fn new() -> Self {
        Self {
            count: 1,
            h: 180.0, s: 0.0, l: 50.0,
            r: 127, g: 127, b: 127,
        }
    }

    pub fn set_count(mut self, count: u32) -> Self {
        self.count = count;
        return self;
    }

    pub fn set_hsl(mut self, h: f32, s: f32, l: f32) -> Self {
      
        self.h = h;
        self.s = s;
        self.l = l;

        self.convert_hsl_to_rgb();

        return self;
    }

    pub fn set_rgb(mut self, r: u8, g: u8, b: u8) -> Self {

        self.r = r;
        self.g = g;
        self.b = b;

        //self.convert_rgb_to_hsl();

        return self;
    }

    pub fn set_color32(mut self, color: Color32) -> Self {
        return self.set_rgb(color.r(), color.g(), color.b());
    }

    fn validate_hsl(&mut self) {
        
        self.h = self.h % 360.0;

        if self.s < 0.0 { self.s = 0.0 };
        if self.s > 100.0 { self.s = 100.0 };

        if self.l < 0.0 { self.l = 0.0 };
        if self.l > 100.0 { self.l = 100.0 };
    }

    fn convert_hsl_to_rgb(&mut self) {

		self.validate_hsl();
		
		let h = self.h / 360.0;
		let s = self.s / 100.0;
		let l = self.l / 100.0;

        if s < 0.001 {
            let gray = (l * 255.0) as u8;
		    self.r = gray;
		    self.g = gray;
		    self.b = gray;
            return;
        }

        let q = if l < 0.5 { 
            l * (1.0 + s)
        } else {
            l + s - l * s
        };
        let p = 2.0 * l - q;

        let r = Self::hue_to_rgb(p, q, h + (1.0/3.0));
	    let g = Self::hue_to_rgb(p, q, h);
		let b = Self::hue_to_rgb(p, q, h - (1.0/3.0));

        self.r = (r * 255.0) as u8;
        self.g = (g * 255.0) as u8;
        self.b = (b * 255.0) as u8;
		
	} 

	fn hue_to_rgb(p: f32, q: f32, mut t: f32) -> f32 {
		
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

    fn convert_rgb_to_hsl(&mut self) {
        
        let mut max = self.r;
        if self.g > max { max = self.g; }
        if self.b > max { max = self.b; }

        let mut min = self.r;
        if self.g < min { min = self.g; }
        if self.b < min { min = self.b; }

		self.h = (max + min) as f32 / 2.0;
		self.s = self.h;
		self.l = self.h;
	
		if (max == min) {
		
			self.h = 0.0;
			self.s = 0.0;
			
		} else {

            let r = (self.r as f32) / 255.0;
            let g = (self.g as f32) / 255.0;
            let b = (self.b as f32) / 255.0;

			let d = (max - min) as f32;
            self.s = if self.l > 0.5 {
                d / (2.0 - (max as f32) - (min as f32))
            } else {
                d / ((max as f32) + (min as f32))
            };

            if max == self.r {
                self.h = (g - b) / d + (if g < b { 6.0 } else { 0.0 });
            } else if max == self.g {
                self.h = (b - r) / d + 2.0;
            } else {
                self.h = (r - g) / d + 4.0;
            }
			self.h = self.h / 6.0;
						
		}	

        println!("{} {} {}", self.h, self.s, self.l);

		self.h = 360.0 * self.h;
		self.s = 100.0 * self.s;
		self.l = 100.0 * self.l;    
        
    }

}

impl Iterator for KolorWheel {
    type Item = Color32;

    fn next(&mut self) -> Option<Color32>{

        if self.count == 0 {
            return None;
        }
        self.count -= 1;

        let color32 = Color32::from_rgb(
            self.r, self.g, self.b
        );

        self.h += 10.0;
        self.convert_hsl_to_rgb();

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
