#![allow(unused)]

use crate::hsl_color::HslColor;
use crate::rgb_color::RgbColor;
use crate::{SpinMode, FadeMode};

pub(crate) struct Spinner<'sr> {
    color: HslColor,    
    count: usize,
    counter: usize,
    spin_mode_hue: SpinMode<'sr>,
    spin_mode_saturation: SpinMode<'sr>,
    spin_mode_lightness: SpinMode<'sr>,
    recalc_request: bool,
    spin_hue: Spin<'sr>,
    spin_saturation: Spin<'sr>,
    spin_lightness: Spin<'sr>,
}

enum Spin<'sp> {
    Still,
    Calculated(f32, Boundary),
    Stored(&'sp [f32]),
}

enum Boundary {
    Circular,
    Percent,
}

impl<'i> Spinner<'i> {

    pub(crate) fn new<T>(color: T, count: usize) -> Self 
    where T: Into<HslColor> {
        Self {
            color: color.into(),
            count,
            counter: 0,
            spin_mode_hue: SpinMode::Unchanged,
            spin_mode_saturation: SpinMode::Unchanged,
            spin_mode_lightness: SpinMode::Unchanged,
            recalc_request: true,
            spin_hue: Spin::Still,
            spin_saturation: Spin::Still,
            spin_lightness: Spin::Still,
        }
    }

    pub(crate) fn color(&self) ->  HslColor {
        self.color
    }

    pub(crate) fn rewind(&mut self) -> &mut Self {
        self.counter = 0;
        self
    }

    pub(crate) fn with_color(&mut self, hsl: HslColor) { 
        self.color = hsl;
        self.recalc_request = true;      
    }

    pub(crate) fn with_hue(&mut self, spin_mode: SpinMode<'i>) {
        self.spin_mode_hue = spin_mode;
        self.recalc_request = true;      
    }

    pub(crate) fn with_saturation(&mut self, spin_mode: SpinMode<'i>) {
        self.spin_mode_saturation = spin_mode;
        self.recalc_request = true;      
    }

    pub(crate) fn with_lightness(&mut self, spin_mode: SpinMode<'i>) {
        self.spin_mode_lightness = spin_mode;
        self.recalc_request = true;      
    }

    fn recalculate(&mut self) {
        
        self.recalc_request = false;

        self.spin_hue = Self::recalculate_channel(
            self.color.h, 
            &self.spin_mode_hue,
            Boundary::Circular,
        );

        self.spin_saturation = Self::recalculate_channel(
            self.color.s, 
            &self.spin_mode_saturation,
            Boundary::Percent,
        );

        self.spin_lightness = Self::recalculate_channel(
            self.color.l, 
            &self.spin_mode_lightness,
            Boundary::Percent,
        );

    }
    
    fn recalculate_channel(value: f32, spin_mode: &SpinMode, boundary: Boundary) -> Spin<'i> {

        match spin_mode {

            SpinMode::Unchanged => {
                return Spin::Still;
            },

            SpinMode::Absolute(target) => {
                println!("---------------- {} -> {}", value, target);
                return Spin::Still;  // TODO: calc
            },

            SpinMode::RelativeIncl(value) => {
                return Spin::Still;  // TODO: calc
            },

            SpinMode::RelativeExcl(value) => {
                return Spin::Still;  // TODO: calc
            },

            SpinMode::Offset(offsets) => {
                return Spin::Still;  // TODO: calc
            },

        }
    }

    pub(crate) fn spin_next(&mut self) ->  HslColor {

        if self.recalc_request { self.recalculate(); }

        self.color.h = self.counter as f32; // TODO: tmp        
        self.color.l += 5.0;

        self.counter += 1;
        self.color
    }

    pub(crate) fn spin_finished(&self) -> bool {
        self.counter >= self.count
    }

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





