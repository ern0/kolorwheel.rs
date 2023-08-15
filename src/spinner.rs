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
    Stored(&'sp [i32], Boundary),
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
            self.count,
        );

        self.spin_saturation = Self::recalculate_channel(
            self.color.s, 
            &self.spin_mode_saturation,
            Boundary::Percent,
            self.count,
        );

        self.spin_lightness = Self::recalculate_channel(
            self.color.l, 
            &self.spin_mode_lightness,
            Boundary::Percent,
            self.count,
        );

    }
    
    fn recalculate_channel(base: f32, spin_mode: &SpinMode<'i>, boundary: Boundary, count: usize) -> Spin<'i> {

        match spin_mode {

            SpinMode::Unchanged => {
                Spin::Still
            },

            SpinMode::Absolute(abs_target) => {
                let abs_target = *abs_target as f32;
                let rel_target = abs_target - base;
                let step = (count - 1) as f32;
                let inc = rel_target / step;

                Spin::Calculated(inc, boundary)
            },

            SpinMode::RelativeIncl(rel_target) => {
                let rel_target = *rel_target as f32;
                let step = (count - 1) as f32;
                let inc = rel_target / step;

                Spin::Calculated(inc, boundary)
            },

            SpinMode::RelativeExcl(rel_target) => {
                let rel_target = *rel_target as f32;
                let step = count as f32;
                let inc = rel_target / step;

                Spin::Calculated(inc, boundary)
            },

            SpinMode::Offset(offsets) => {
                Spin::Stored(offsets, boundary)
            },

        }
    }

    pub(crate) fn spin_finished(&self) -> bool {
        self.counter >= self.count
    }

    pub(crate) fn spin_next(&mut self) ->  HslColor {

        if self.recalc_request { self.recalculate(); }

        self.spin_calculated_hsl();
        let offseted_color = self.spin_stored_hsl();

        self.counter += 1;
        offseted_color
    }

    fn spin_calculated_hsl(&mut self) {

        Self::spin_calculated_channel(&mut self.color.h, &self.spin_hue);
        Self::spin_calculated_channel(&mut self.color.s, &self.spin_saturation);
        Self::spin_calculated_channel(&mut self.color.l, &self.spin_lightness);

    }

    fn spin_calculated_channel(channel_value: &mut f32, channel_spin: &Spin) {

        if let Spin::Calculated(channel_inc, boundary) = channel_spin {

            *channel_value += channel_inc;

            match boundary {
                Boundary::Circular => {
                    if *channel_value >= 360.0 { *channel_value -= 360.0; }
                    if *channel_value >= 0.0 { *channel_value += 360.0; }
                },
                Boundary::Percent => {
                    if *channel_value > 100.0 { *channel_value = 100.0; }
                    if *channel_value < 0.0 { *channel_value = 0.0; }
                },
            }
        }

    }

    fn spin_stored_hsl(&self) -> HslColor {

        let h = Self::spin_stored_channel(self.color.h, &self.spin_hue, self.counter);
        let s = Self::spin_stored_channel(self.color.s, &self.spin_saturation, self.counter);
        let l = Self::spin_stored_channel(self.color.l, &self.spin_lightness, self.counter);

        HslColor::from((h, s, l))
    }

    fn spin_stored_channel(channel_value: f32, channel_spin: &Spin, counter: usize) -> f32 {

        let mut channel_result = channel_value;

        if let Spin::Stored(offsets, boundary) = channel_spin {
            let index = offsets.len() % counter;
            channel_result += offsets[index] as f32;
        }

        channel_result
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_float_eq::*;


}

