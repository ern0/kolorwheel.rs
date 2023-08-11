#![allow(unused)]

mod hsl_color;
mod rgb_color;
mod convert_hsl_to_rgb;
mod convert_rgb_to_hsl;

use hsl_color::HslColor;
use rgb_color::RgbColor;

pub struct KolorWheel<'a> {
    color: HslColor,    
    count: usize,
    spin_hue: Spin<'a>,
    spin_saturation: Spin<'a>,
    spin_lightness: Spin<'a>,
}

pub enum Spin<'a> {
    Unchanged,
    Absolute(i32),
    Relative(i32),
    Offset(&'a [i32]),
    Stored(Vec<i32>),
}

impl<'a> KolorWheel<'a> {

    pub fn new<T>(color: T, count: usize) -> Self 
    where T: Into<HslColor> {
        Self {
            color: color.into(),
            count,
            spin_hue: Spin::Unchanged,
            spin_saturation: Spin::Unchanged,
            spin_lightness: Spin::Unchanged,
        }
    }

    pub fn spin_hsl<T>(&mut self, target: T) -> &mut Self 
    where T: Into<HslColor> {       
        let hsl: HslColor = target.into();
        self.spin_hue(Spin::Absolute(hsl.h as i32));
        self.spin_saturation(Spin::Absolute(hsl.s as i32));
        self.spin_lightness(Spin::Absolute(hsl.l as i32));
        self
    }

    pub fn spin_hue(&mut self, spin: Spin<'a>) -> &mut Self {
        self.spin_hue = spin;
        self
    }

    pub fn spin_saturation(&mut self, spin: Spin<'a>) -> &mut Self {
        self.spin_saturation = spin;
        self
    }

    pub fn spin_lightness(&mut self, spin: Spin<'a>) -> &mut Self {
        self.spin_lightness = spin;
        self
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_float_eq::*;

    #[test]
    fn tst() {
        let kw = KolorWheel::new(HslColor::new(0, 100, 50), 8)
            .spin_hsl(HslColor::new(90, 100, 50))
            .spin_hue(Spin::Absolute(90))
            .spin_saturation(Spin::Relative(-10))
            .spin_lightness(Spin::Offset(&[0, 10]))
            //.skip_first()

        ;
    }
}

/*

    fn slice_u32_to_vec_i32(values: &[u32]) -> Vec<i32> {

        let mut vec_values: Vec<i32> = Vec::new();

        for uvalue in values { 
            let ivalue = *uvalue as i32;
            vec_values.push(ivalue) 
        };

        return vec_values;
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
