#![allow(unused)]

mod hsl_color;
mod rgb_color;
mod convert_hsl_to_rgb;
mod convert_rgb_to_hsl;
mod spinner;

use hsl_color::HslColor;
use rgb_color::RgbColor;
use crate::spinner::{Spinner, SpinMode, FadeMode};

pub struct KolorWheel<'kw> {
    index: usize,
    spinner_vec: Vec<Spinner<'kw>>,
}

impl<'kw> KolorWheel<'kw> {

    pub fn new<T>(color: T, count: usize) -> Self 
    where T: Into<HslColor> {

        let mut vec = Vec::new();
        let mut spinner = Spinner::new(color, count);
        vec.push(spinner);
        Self {
            index: 0,
            spinner_vec: vec,
        }
    }

    pub fn actual_spinner(&mut self) -> &mut Spinner<'kw> {
        &mut self.spinner_vec[self.index]
    }

    pub fn color<T>(&mut self, target: T) -> &mut Self 
    where T: Into<HslColor> {       
        self.actual_spinner().color = target.into();
        self
    }

    pub fn with_hue(&mut self, spin_mode: SpinMode<'kw>) -> &mut KolorWheel<'kw> {
        self.actual_spinner().spin_hue = spin_mode;
        self
    }

    pub fn with_saturation(&mut self, spin_mode: SpinMode<'kw>) -> &mut KolorWheel<'kw> {
        self.actual_spinner().spin_saturation = spin_mode;
        self
    }

    pub fn with_lightness(&mut self, spin_mode: SpinMode<'kw>) -> &mut Self {
        self.actual_spinner().spin_lightness = spin_mode;
        self
    }

    pub fn fade(&mut self, fade_mode: FadeMode) -> &mut Self {

        let spinner = self.actual_spinner();

        match fade_mode {
            FadeMode::Color(hsl_color) => {
                spinner.spin_hue = SpinMode::Absolute(hsl_color.h as i32);
                spinner.spin_saturation = SpinMode::Absolute(hsl_color.s as i32);
                spinner.spin_lightness = SpinMode::Absolute(hsl_color.l as i32);
            },
            FadeMode::Gray(percent) => {
                spinner.spin_saturation = SpinMode::Absolute(0);
                spinner.spin_lightness = SpinMode::Absolute(percent);
            },
            FadeMode::Black => {
                spinner.spin_saturation = SpinMode::Absolute(0);
                spinner.spin_lightness = SpinMode::Absolute(0);
            },
            FadeMode::White => {
                spinner.spin_saturation = SpinMode::Absolute(0);
                spinner.spin_lightness = SpinMode::Absolute(100);
            },
        };

        self
    }    

    pub fn fork(&mut self, count: usize) -> &mut Self {

        let mut spinner = self.actual_spinner().clone();
        self.index += 1;

        spinner.count = count;
        spinner.skip_first = true;
        self.spinner_vec.push(spinner);

        self
    }

    pub fn spin<T: From<HslColor>>(&mut self, callback: &dyn Fn(T)) {

        let mut level = 0;
        loop {

            let result = self.spinner_vec[level].spin_next_result();
            match result {

                Some(color) => {

                    callback(color.into());

                    if self.spinner_vec.len() > level + 1 {

                        level += 1;

                        let mut spinner = &mut self.spinner_vec[level];                        
                        spinner.counter = 0;                        
                        spinner.color = color;
                    }
                },

                None => {
                    if level == 0 { return; }
                    level -= 1;
                },
            }

        } // loop
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_float_eq::*;

    #[test]
    fn tst() {

        println!(">>>>>>>>>>>>>>>>>>>>");

        let kw = KolorWheel::new(HslColor::new(0, 100, 50), 4)
            .fade(FadeMode::Color(HslColor::new(0, 100, 100)))
            .fade(FadeMode::Gray(50))
            .fade(FadeMode::Black)
            .fade(FadeMode::White)
            .with_hue(SpinMode::Absolute(90))
            .with_saturation(SpinMode::RelativeIncl(-10))
            .with_lightness(SpinMode::Offset(&[0, 10]))
            .fork(3)
            .spin(&|res: HslColor| {
                println!("-------------{:?}", res);
            })
        ;
        println!("<<<<<<<<<<<<<<<<<<<<");

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

}

*/
