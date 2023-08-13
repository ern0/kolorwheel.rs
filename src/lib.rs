#![allow(unused)]

mod hsl_color;
mod rgb_color;
mod convert_hsl_to_rgb;
mod convert_rgb_to_hsl;
mod spinner;

use hsl_color::HslColor;
use rgb_color::RgbColor;
use crate::spinner::{Spinner, SpinMode};

#[derive(Clone)]
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

    // pub fn with_hsl<T>(&mut self, target: T) -> &mut Self 
    // where T: Into<HslColor> {       
    //     let hsl: HslColor = target.into();
    //     self.with_hue(Spin::Absolute(hsl.h as i32));
    //     self.with_saturation(Spin::Absolute(hsl.s as i32));
    //     self.with_lightness(Spin::Absolute(hsl.l as i32));
    //     self
    // }

    pub fn test_value(&mut self, value: i32) -> &mut Self {

        self.spinner_vec[self.index].test_value(value);
        self
    }

    // pub fn with_hue(&mut self, spin: Spin<'a>) -> &mut Self {
    //     self.spin_hue = spin;
    //     self
    // }

    // pub fn with_saturation(&mut self, spin: Spin<'a>) -> &mut Self {
    //     self.spin_saturation = spin;
    //     self
    // }

    // pub fn with_lightness(&mut self, spin: Spin<'a>) -> &mut Self {
    //     self.spin_lightness = spin;
    //     self
    // }

    pub fn chain(&mut self, count: usize) -> &mut Self {

        let mut spinner = self.spinner_vec[self.index].clone();
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
                        self.spinner_vec[level].counter = 0;
                    }
                },

                None => {
                    if level == 0 { return; }
                    level -= 1;
                }
            }
        }
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
            // .with_hsl(HslColor::new(0, 100, 100))
            // .with_hue(Spin::Absolute(90))
            // .with_saturation(Spin::RelativeIncl(-10))
            // .with_lightness(Spin::Offset(&[0, 10]))
            //.skip_first()
            .test_value(111)
            .chain(3)
            .test_value(222)
            .spin(&|res: HslColor| {

                println!("-------------{:?}", res);

                // let inner = KolorWheel::new(res, 2)
                //     .skip_first()
                //     .spin(&|ires: RgbColor| {
                //         println!("  -------------{:?}", ires);
                //     })
                // ;

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
