#![allow(unused)]

use crate::hsl_color::HslColor;
use crate::rgb_color::RgbColor;

#[derive(Clone)]
pub struct Spinner<'sp> {
    pub color: HslColor,    
    pub count: usize,
    pub counter: usize,
    pub skip_first: bool,
    spin_hue: SpinMode<'sp>,
    spin_saturation: SpinMode<'sp>,
    spin_lightness: SpinMode<'sp>,
}

#[derive(Clone)]
pub enum SpinMode<'sl> {
    Unchanged,
    Absolute(i32),
    RelativeIncl(i32),
    RelativeExcl(i32),
    Offset(&'sl [i32]),
}

impl<'a> Spinner<'a> {

    pub fn new<T>(color: T, count: usize) -> Self 
    where T: Into<HslColor> {
        Self {
            color: color.into(),
            count,
            counter: 0,
            skip_first: false,
            spin_hue: SpinMode::Unchanged,
            spin_saturation: SpinMode::Unchanged,
            spin_lightness: SpinMode::Unchanged,
        }
    }

    pub fn test_value(&mut self, value: i32) {

        self.color.h = value as f32;
        self.color.s = value as f32;
        self.color.l = value as f32;

    }

    pub fn spin_next_result(&mut self) -> Option<HslColor> {

        if self.counter == self.count {
            return None;
        }

        self.color.h = self.counter as f32; // TODO: tmp
        
        let result = self.color;
        self.color.l += 5.0;

        self.counter += 1;

        if self.counter == 1 && self.skip_first { 
            return self.spin_next_result(); 
        }

        return Some(result);
    }


    // pub fn with_hue(&mut self, spin: SpinMode<'a>) -> &mut Self {
    //     self.spin_hue = spin;
    //     self
    // }

    // pub fn with_saturation(&mut self, spin: SpinMode<'a>) -> &mut Self {
    //     self.spin_saturation = spin;
    //     self
    // }

    // pub fn with_lightness(&mut self, spin: SpinMode<'a>) -> &mut Self {
    //     self.spin_lightness = spin;
    //     self
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_float_eq::*;


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

}

*/
