#![allow(unused)]

use egui::Color32;

pub struct KolorWheel {
    count: u32,
    countf: f32,
    h: f32, s: f32, l: f32,
    r: u8, g: u8, b: u8,
    h_inc: f32,
    h_spin: Spin,
    h_counter: usize,
    s_inc: f32,
    s_spin: Spin,
    s_counter: usize,
    l_inc: f32,
    l_spin: Spin,
    l_counter: usize,
}

enum Spin {
    IncrementOnly,
    AbsoluteVec(Vec<u32>),
    OffsetVec(Vec<u32>),
}

impl KolorWheel {

    pub fn new() -> Self {
        Self {
            count: 1, 
            countf: 1.0,
            h: 180.0, 
            s: 0.0, 
            l: 50.0,
            r: 127, 
            g: 127, 
            b: 127,
            h_inc: 0.0, 
            h_spin: Spin::IncrementOnly, 
            h_counter: 0,
            s_inc: 0.0, 
            s_spin: Spin::IncrementOnly, 
            s_counter: 0,
            l_inc: 0.0,
            l_spin: Spin::IncrementOnly, 
            l_counter: 0,
        }
    }

    pub fn set_count(mut self, count: u32) -> Self {
        self.count = count;
        self.countf = count as f32;
        return self;
    }

    pub fn set_hsl(mut self, h: f32, s: f32, l: f32) -> Self {
      
        self.h = h;
        self.s = s;
        self.l = l;

        self.normalize_hsl();
        self.convert_hsl_to_rgb();

        return self;
    }

    pub fn set_rgb(mut self, r: u8, g: u8, b: u8) -> Self {

        self.r = r;
        self.g = g;
        self.b = b;

        self.convert_rgb_to_hsl();
        self.normalize_hsl();

        return self;
    }

    pub fn set_rgb_hex(mut self, hex: &str) -> Self {

        let mut hexb = hex.as_bytes();

        if hexb.len() == 0 {
            return self.set_rgb_hex_error();
        }
        if hexb[0] == b'#' {
            hexb = &hexb[1..];
        }
        if hexb.len() == 3 {
            self.set_rgb_hex_parse(&[hexb[0], hexb[0], hexb[1], hexb[1], hexb[2], hexb[2],]);
        }
        if hexb.len() == 6 {
            self.set_rgb_hex_parse(hexb);
        }        

        return self;
    }

    fn set_rgb_hex_parse(&mut self, hexb: &[u8]) {

        let r_hi = Self::set_rgb_hex_parse_digit(hexb[0]);
        if let Err(_) = r_hi { return; }
        let r_lo = Self::set_rgb_hex_parse_digit(hexb[1]);
        if let Err(_) = r_lo { return; }

        let g_hi = Self::set_rgb_hex_parse_digit(hexb[2]);
        if let Err(_) = g_hi { return; }
        let g_lo = Self::set_rgb_hex_parse_digit(hexb[3]);
        if let Err(_) = g_lo { return; }

        let b_hi = Self::set_rgb_hex_parse_digit(hexb[4]);
        if let Err(_) = b_hi { return; }
        let b_lo = Self::set_rgb_hex_parse_digit(hexb[5]);
        if let Err(_) = b_lo { return; }

        self.r = (r_hi.unwrap() << 4) + r_lo.unwrap();
        self.g = (g_hi.unwrap() << 4) + g_lo.unwrap();
        self.b = (b_hi.unwrap() << 4) + b_lo.unwrap();

    }

    fn set_rgb_hex_parse_digit(digit: u8) -> Result<u8, ()> {

        if digit >= b'0' && digit <= b'9' {
            return Ok(digit - b'0');
        }
        if digit >= b'a' && digit <= b'f' {
            return Ok(10 + digit - b'a');
        }
        if digit >= b'A' && digit <= b'F' {
            return Ok(10 + digit - b'A');
        }

        return Err(());
    }

    fn set_rgb_hex_error(self) -> Self {
        // error handling: silent ignore
        return self;
    }

    pub fn set_color32(self, color: Color32) -> Self {
        return self.set_rgb(color.r(), color.g(), color.b());
    }

    fn normalize_hsl(&mut self) {
        
        self.h = self.h % 360.0;

        if self.s < 0.0 { self.s = 0.0 };
        if self.s > 100.0 { self.s = 100.0 };

        if self.l < 0.0 { self.l = 0.0 };
        if self.l > 100.0 { self.l = 100.0 };
    }

    fn convert_hsl_to_rgb(&mut self) {
		
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
            l + s - (l * s)
        };
        let p = (2.0 * l) - q;

        let r = Self::hue_to_rgb(p, q, h + (1.0/3.0));
	    let g = Self::hue_to_rgb(p, q, h);
		let b = Self::hue_to_rgb(p, q, h - (1.0/3.0));

        let r = (r * 12000.0).round() / 12000.0;
        let g = (g * 12000.0).round() / 12000.0;
        let b = (b * 12000.0).round() / 12000.0;

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
            return p + ((q - p) * 6.0 * t);
        }
		if t < (1.0/2.0) {
            return q;
        }
		if t < (2.0/3.0) {
            return p + ((q - p) * ((2.0/3.0) - t) * 6.0);
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

		self.h = (max as f32 + min as f32) / 255.0 / 2.0;
		self.s = self.h;
		self.l = self.h;
	
		if max == min {
		
			self.h = 0.0;
			self.s = 0.0;
			
		} else {

            let r = (self.r as f32) / 255.0;
            let g = (self.g as f32) / 255.0;
            let b = (self.b as f32) / 255.0;
            let minf = (min as f32) / 255.0;
            let maxf = (max as f32) / 255.0;

			let d = maxf - minf;
            self.s = if self.l > 0.5 {
                d / (2.0 - maxf - minf)
            } else {
                d / (maxf + minf)
            };

            if max == self.r {
                self.h = (g - b) / (d + (if g < b { 6.0 } else { 0.0 }));
            } else if max == self.g {
                self.h = ((b - r) / d) + 2.0;
            } else {
                self.h = ((r - g) / d) + 4.0;
            }
			self.h = self.h / 6.0;
						
		}	

		self.h *= 360.0;
		self.s *= 100.0;
		self.l *= 100.0;    
        
    }

    pub fn hue_abs(mut self, amount: u32) -> Self {
        self.h_inc = (amount as f32 - self.h) / self.countf;
        return self;
    }

    pub fn hue_rel(mut self, amount: u32) -> Self {
        self.h_inc = amount as f32 / self.countf;
        return self;
    }

    pub fn hue_values(mut self, values: &[u32]) -> Self {
        self.h_spin = Spin::AbsoluteVec(Vec::from(values));
        return self;
    }

    pub fn sat_abs(mut self, amount: u32) -> Self {
        self.s_inc = (amount as f32 - self.s) / self.countf;
        return self;
    }

    pub fn sat_rel(mut self, amount: u32) -> Self {
        self.s_inc = amount as f32 / self.countf;
        return self;
    }

    pub fn sat_values(mut self, values: &[u32]) -> Self {
        self.s_spin = Spin::AbsoluteVec(Vec::from(values));
        return self;
    }

    pub fn lit_abs(mut self, amount: u32) -> Self {
        self.l_inc = (amount as f32 - self.h) / self.countf;
        return self;
    }

    pub fn lit_values(mut self, values: &[u32]) -> Self {
        self.l_spin = Spin::AbsoluteVec(Vec::from(values));
        return self;
    }

    pub fn lit_offsets(mut self, offsets: &[u32]) -> Self {
        self.l_spin = Spin::OffsetVec(offsets.to_vec());
        return self;
    }

    pub fn lit_rel(mut self, amount: u32) -> Self {
        self.l_inc = amount as f32 / self.countf;
        return self;
    }

    fn next_spin(values: Vec<u32>, counter: &mut usize) -> f32 {

        let mut result = values[*counter] as f32;
        *counter += 1;
        if *counter == values.len() {
            *counter = 0;
        }

        return result;
    }

    fn next_pre_channel(channel_value: &mut f32, channel_spin: &Spin, channel_counter: &mut usize) {

        if let Spin::AbsoluteVec(values) = channel_spin {
            *channel_value = Self::next_spin(values.to_vec(), channel_counter);
        }

    }

    fn next_post_channel(channel_value: &mut f32, channel_inc: f32, channel_spin: &Spin, channel_counter: &mut usize) {

        match channel_spin {
            Spin::IncrementOnly => {
                *channel_value += channel_inc;
            },
            Spin::OffsetVec(values) => {
                *channel_value += channel_inc;;
                *channel_value += Self::next_spin(values.to_vec(), channel_counter);
            },
            _ => {},
        }

    }

    fn next_pre(&mut self) {
        Self::next_pre_channel(&mut self.h, &self.h_spin, &mut self.h_counter);
        Self::next_pre_channel(&mut self.s, &self.s_spin, &mut self.s_counter);
        Self::next_pre_channel(&mut self.l, &self.l_spin, &mut self.l_counter);
    }

    fn next_post(&mut self) {
        Self::next_post_channel(&mut self.h, self.h_inc, &self.h_spin, &mut self.h_counter);
        Self::next_post_channel(&mut self.s, self.s_inc, &self.s_spin, &mut self.s_counter);
        Self::next_post_channel(&mut self.l, self.l_inc, &self.l_spin, &mut self.l_counter);
    }


}

impl Iterator for KolorWheel {
    type Item = Color32;

    fn next(&mut self) -> Option<Color32>{

        if self.count == 0 {
            return None;
        }
        self.count -= 1;

        self.next_pre();
        self.normalize_hsl();
        self.convert_hsl_to_rgb();

        let color32 = Color32::from_rgb(
            self.r, self.g, self.b
        );

        if self.count > 0 {
            self.next_post();
        }

        self.normalize_hsl();
        self.convert_hsl_to_rgb();

        return Some(color32);
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_float_eq::*;

    #[test]
    fn hsl_to_rgb_black() {
        let kw = KolorWheel::new().set_hsl(0.0, 0.0, 0.0);
        assert_eq!((kw.r, kw.g, kw.b,), (0, 0, 0));     
    }

    #[test]
    fn hsl_to_rgb_white() {
        let kw = KolorWheel::new().set_hsl(0.0, 100.0, 100.0);
        assert_eq!((kw.r, kw.g, kw.b,), (255, 255, 255));     
    }

    #[test]
    fn hsl_to_rgb_gray() {
        let kw = KolorWheel::new().set_hsl(240.0, 0.0, 40.0);
        assert!(kw.r == kw.g);     
        assert!(kw.g == kw.b);     
        assert!(kw.r > 1);     
    }

    #[test]
    fn hsl_to_rgb_red() {
        let kw = KolorWheel::new().set_hsl(0.0, 100.0, 50.0);
        assert_eq!((kw.r, kw.g, kw.b,), (255, 0, 0));     
    }

    #[test]
    fn hsl_to_rgb_green() {
        let kw = KolorWheel::new().set_hsl(120.0, 100.0, 50.0);
        assert_eq!((kw.r, kw.g, kw.b,), (0, 255, 0));     
    }

    #[test]
    fn hsl_to_rgb_cyan() {
        let kw = KolorWheel::new().set_hsl(180.0, 100.0, 50.0);
        assert_eq!((kw.r, kw.g, kw.b,), (0, 255, 255));     
    }

    #[test]
    fn hsl_to_rgb_blue() {
        let kw = KolorWheel::new().set_hsl(240.0, 100.0, 50.0);
        assert_eq!((kw.r, kw.g, kw.b,), (0, 0, 255));     
    }

    #[test]
    fn hsl_to_rgb_overflow_cyan() {
        let kw = KolorWheel::new().set_hsl(360.0 + 180.0, 100.0, 50.0);
        assert_eq!((kw.r, kw.g, kw.b,), (0, 255, 255));     
    }

    #[test]
    fn hsl_to_rgb_underflow_blue() {
        let kw = KolorWheel::new().set_hsl(-120.0, 100.0, 50.0);
        assert_eq!((kw.r, kw.g, kw.b,), (0, 0, 255));     
    }

    #[test]
    fn hsl_to_rgb_light_blue() {
        let kw = KolorWheel::new().set_hsl(240.0, 100.0, 90.0);
        assert!(kw.r == kw.g);     
        assert!(kw.b > kw.r);
        assert!(kw.b > 240);
    }

    #[test]
    fn rgb_to_hsl_black() {
        let kw = KolorWheel::new().set_rgb(0, 0, 0);
        assert_f32_near!(kw.h, 0.0);
        assert_f32_near!(kw.s, 0.0);
        assert_f32_near!(kw.l, 0.0);
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

}
