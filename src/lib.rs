#![allow(unused)]

mod hsl_color;
mod rgb_color;
mod convert_hsl_to_rgb;
mod convert_rgb_to_hsl;

use hsl_color::HslColor;

#[derive(Default)]
pub struct KolorWheel {
    color: HslColor,    
    count: u32,

}

enum Spin {
    Unchanged,
    Calculated(f32),
    Stored(Vec<i32>),
}

enum Offset {
    Zero,
    OffsetVec(Vec<i32>),
}


impl KolorWheel {

    pub fn new() -> Self {
        Self {

            count: 1, 
            ..Default::default()

            // countf: 1.0,
            // h: 180.0, 
            // s: 0.0, 
            // l: 50.0,
            // saved_h: 0.0,
            // saved_s: 0.0, 
            // saved_l: 0.0,
            // r: 127, 
            // g: 127, 
            // b: 127,
            // h_spin: Spin::Unchanged, 
            // h_spin_counter: 0,
            // h_offset: Offset::Zero,
            // h_offset_counter: 0,
            // s_spin: Spin::Unchanged, 
            // s_spin_counter: 0,
            // s_offset: Offset::Zero,
            // s_offset_counter: 0,
            // l_spin: Spin::Unchanged, 
            // l_spin_counter: 0,
            // l_offset: Offset::Zero,
            // l_offset_counter: 0,
        }
    }




/*
    pub fn set_count(mut self, count: u32) -> Self {
        self.count = count;
        self.countf = count as f32;
        return self;
    }

    pub fn set_hsl(mut self, h: u32, s: u32, l: u32) -> Self {
      
        self.h = h as f32;
        self.s = s as f32;
        self.l = l as f32;

        self.normalize_hsl();
        self.convert_hsl_to_rgb();

        return self;
    }

    pub fn set_rgb(mut self, r: u8, g: u8, b: u8) -> Self {

        self.r = r;
        self.g = g;
        self.b = b;

        //self.convert_rgb_to_hsl();
        self.normalize_hsl();

        return self;
    }

    pub fn set_rgb_f(mut self, r: f32, g: f32, b: f32) -> Self {
        self.set_rgb(
            (r * 255.0) as u8, 
            (g * 255.0) as u8, 
            (b * 255.0) as u8,
        )
    }

    pub fn set_rgb_fa(mut self, rgb: [f32; 3]) -> Self {
        self.set_rgb_f(
            rgb[0], 
            rgb[1], 
            rgb[2]
        )
    }

    pub fn gradient(mut self, target: KolorWheel) -> Self {
        return self
            .hue_abs(target.h as u32)
            .sat_abs(target.s as u32)
            .lit_abs(target.l as u32)
        ;
    }

    fn normalize_hsl(&mut self) {
        
        self.h = self.h % 360.0;
        if self.h < 0.0 { self.h += 360.0 };

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


    fn slice_u32_to_vec_i32(values: &[u32]) -> Vec<i32> {

        let mut vec_values: Vec<i32> = Vec::new();

        for uvalue in values { 
            let ivalue = *uvalue as i32;
            vec_values.push(ivalue) 
        };

        return vec_values;
    }

    pub fn hue_abs(mut self, amount: u32) -> Self {
        let inc = (amount as f32 - self.h) / self.countf;
        self.h_spin = Spin::Calculated(inc);
        return self;
    }

    pub fn hue_reli(mut self, amount: i32) -> Self {
        let inc = amount as f32 / (self.countf - 1.0);
        self.h_spin = Spin::Calculated(inc);
        return self;
    }

    pub fn hue_relx(mut self, amount: i32) -> Self {
        let inc = amount as f32 / self.countf;
        self.h_spin = Spin::Calculated(inc);
        return self;
    }

    pub fn hue_vals(mut self, values: &[u32]) -> Self {
        self.h_spin = Spin::Stored(Self::slice_u32_to_vec_i32(values));
        return self;
    }

    pub fn hue_offs(mut self, offsets: &[i32]) -> Self {
        self.h_offset = Offset::OffsetVec(offsets.to_vec());
        return self;
    }

    pub fn sat_abs(mut self, amount: u32) -> Self {
        let inc = (amount as f32 - self.s) / self.countf;
        self.s_spin = Spin::Calculated(inc);
        return self;
    }

    pub fn sat_reli(mut self, amount: i32) -> Self {
        let inc = amount as f32 / (self.countf - 1.0);
        self.s_spin = Spin::Calculated(inc);
        return self;
    }

    pub fn sat_relx(mut self, amount: i32) -> Self {
        let inc = amount as f32 / self.countf;
        self.s_spin = Spin::Calculated(inc);
        return self;
    }

    pub fn sat_vals(mut self, values: &[u32]) -> Self {
        self.s_spin = Spin::Stored(Self::slice_u32_to_vec_i32(values));
        return self;
    }

    pub fn sat_offs(mut self, offsets: &[i32]) -> Self {
        self.s_offset = Offset::OffsetVec(offsets.to_vec());
        return self;
    }

    pub fn lit_abs(mut self, amount: u32) -> Self {
        let inc = (amount as f32 - self.l) / self.countf;
        self.l_spin = Spin::Calculated(inc);
        return self;
    }

    pub fn lit_reli(mut self, amount: i32) -> Self {
        let inc = amount as f32 / (self.countf - 1.0);
        self.l_spin = Spin::Calculated(inc);
        return self;
    }

    pub fn lit_relx(mut self, amount: i32) -> Self {
        let inc = amount as f32 / self.countf;
        self.l_spin = Spin::Calculated(inc);
        return self;
    }

    pub fn lit_vals(mut self, values: &[u32]) -> Self {
        self.l_spin = Spin::Stored(Self::slice_u32_to_vec_i32(values));
        return self;
    }

    pub fn lit_offs(mut self, offsets: &[i32]) -> Self {
        self.l_offset = Offset::OffsetVec(offsets.to_vec());
        return self;
    }

    fn next_from_vector(values: Vec<i32>, counter: &mut usize) -> f32 {

        let mut result = values[*counter] as f32;
        *counter += 1;
        if *counter == values.len() {
            *counter = 0;
        }

        return result;
    }

    fn spin_stored_hsl(&mut self) {

        Self::spin_stored_channel(&mut self.h, &self.h_spin, &mut self.h_spin_counter);
        Self::spin_stored_channel(&mut self.s, &self.s_spin, &mut self.s_spin_counter);
        Self::spin_stored_channel(&mut self.l, &self.l_spin, &mut self.l_spin_counter);
    }

    fn spin_stored_channel(channel_value: &mut f32, channel_spin: &Spin, channel_counter: &mut usize) {

        if let Spin::Stored(values) = channel_spin {
            *channel_value = Self::next_from_vector(values.to_vec(), channel_counter);
        }
    }

    fn spin_calculated_hsl(&mut self) {

        Self::spin_calculated_channel(&mut self.h, &self.h_spin, &mut self.h_spin_counter);
        Self::spin_calculated_channel(&mut self.s, &self.s_spin, &mut self.s_spin_counter);
        Self::spin_calculated_channel(&mut self.l, &self.l_spin, &mut self.l_spin_counter);
    }

    fn spin_calculated_channel(channel_value: &mut f32, channel_spin: &Spin, channel_counter: &mut usize) {

        if let Spin::Calculated(channel_inc) = channel_spin {
            *channel_value += channel_inc;
        }
    }

    fn offset_hsl(&mut self) {

        Self::offset_channel(&mut self.h, &self.h_offset, &mut self.h_offset_counter);
        Self::offset_channel(&mut self.s, &self.s_offset, &mut self.s_offset_counter);
        Self::offset_channel(&mut self.l, &self.l_offset, &mut self.l_offset_counter);
    }

    fn offset_channel(channel_value: &mut f32, channel_offset: &Offset, channel_counter: &mut usize) {

        if let Offset::OffsetVec(values) = channel_offset {
            *channel_value += Self::next_from_vector(values.to_vec(), channel_counter);
        }
    }

    fn save_hsl(&mut self) {
        self.saved_h = self.h;
        self.saved_s = self.s;
        self.saved_l = self.l;
    }

    fn restore_hsl(&mut self) {
        self.h = self.saved_h;
        self.s = self.saved_s;
        self.l = self.saved_l;
    }

}

impl Iterator for KolorWheel {
    type Item = Color;

    fn next(&mut self) -> Option<Color>{

        if self.count == 0 {
            return None;
        }
        self.count -= 1;

        self.spin_stored_hsl();
        self.normalize_hsl();
        self.save_hsl();
        self.offset_hsl();
        self.normalize_hsl();
        self.convert_hsl_to_rgb();
        self.restore_hsl();

        let color = Color {
            r: self.r, g: self.g, b: self.b
        };

        if self.count > 0 {
            self.spin_calculated_hsl();
        }

        return Some(color);
    }
*/
}



#[cfg(test)]
mod tests {
    use super::*;
    use assert_float_eq::*;

    #[test]
    fn tst() {
    }
}
