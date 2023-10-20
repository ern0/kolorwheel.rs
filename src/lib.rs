/*! ## What is it?

This crate is designed 
to make it easy 
to create palettes for GUI applications.

> With a slightly different API, for a slightly different platform:
[KolorWheel.js](https://github.com/ern0/kolorwheel.js/)

The way of creating a palette is 
to specify a base color 
and some parameters 
that modifies the H, S, L values
in the given *spin mode* and steps.

*Full documentation is available at the project's GitHub page:
[KolorWheel.rs](https://github.com/ern0/kolorwheel.js/)*

*/

#[doc(hidden)]
pub mod hsl_color;
#[doc(hidden)]
pub mod rgb_color;

mod convert_hsl_to_rgb;
mod convert_rgb_to_hsl;
mod spinner;

pub use hsl_color::HslColor;
pub use rgb_color::RgbColor;

use std::vec::Vec;
use crate::spinner::Spinner;

/// The color wheel object, which emits series of
/// [`HslColor`](HslColor) colors upon spin
pub struct KolorWheel {
    index: usize,
    spinner_vec: Vec<Spinner>,
    level: usize,
    top_level: usize,
}

pub enum SpinMode<'m> {
    Still,
    Absolute(i32),
    RelativeIncl(i32),
    RelativeExcl(i32),
    Offset(&'m [i32]),
}

pub enum SpinMacro {
    GradientColor(HslColor),
    FadeToGray(i32),
    FadeToBlack,
    FadeToWhite,
}

impl Iterator for KolorWheel {
    type Item = HslColor;

    fn next(&mut self) -> Option<HslColor> {
        self.spin_iter()
    }
}

impl KolorWheel {

    pub fn new<T>(color: T, count: usize) -> Self 
    where T: Into<HslColor> {

        let mut vec = Vec::new();
        let spinner = Spinner::new(color, count);
        vec.push(spinner);
        Self {
            index: 0,
            spinner_vec: vec,
            level: 0,
            top_level: 0,
        }
    }

    fn actual_spinner(&mut self) -> &mut Spinner {
        &mut self.spinner_vec[self.index]
    }

    pub fn with_hue(&mut self, spin_mode: SpinMode) -> &mut Self {
        self.actual_spinner().with_hue(spin_mode);
        self
    }

    pub fn with_saturation(&mut self, spin_mode: SpinMode) -> &mut Self {
        self.actual_spinner().with_saturation(spin_mode);
        self
    }

    pub fn with_lightness(&mut self, spin_mode: SpinMode) -> &mut Self {
        self.actual_spinner().with_lightness(spin_mode);
        self
    }

    pub fn with_macro(&mut self, spin_macro: SpinMacro) -> &mut Self {

        match spin_macro {
            SpinMacro::GradientColor(hsl_color) => {
                self.with_hue(SpinMode::Absolute(hsl_color.h as i32));
                self.with_saturation(SpinMode::Absolute(hsl_color.s as i32));
                self.with_lightness(SpinMode::Absolute(hsl_color.l as i32));
            },
            SpinMacro::FadeToGray(percent) => {
                self.with_saturation(SpinMode::Absolute(0));
                self.with_lightness(SpinMode::Absolute(percent));
            },
            SpinMacro::FadeToBlack => {
                self.with_saturation(SpinMode::Absolute(0));
                self.with_lightness(SpinMode::Absolute(0));
            },
            SpinMacro::FadeToWhite => {
                self.with_saturation(SpinMode::Absolute(0));
                self.with_lightness(SpinMode::Absolute(100));
            },
        };

        self
    }    

    pub fn fork(&mut self, count: usize) -> &mut Self {

        let color = self.actual_spinner().color();

        let spinner = Spinner::new(color, count);
        self.spinner_vec.push(spinner);
        self.top_level += 1;
        self.index += 1;

        self
    }

    pub fn spin_iter(&mut self) -> Option<HslColor> {

        loop {

            let spinner = &mut self.spinner_vec[self.level];

            if spinner.spin_finished() {
                if self.level == 0 { 
                    return None;
                }
                self.level -= 1;
                continue;
            }
                
            let color = spinner.spin_next();

            if self.level == self.top_level {  // render only top level
                return Some(color);
            }

            self.level += 1;
            let child = &mut self.spinner_vec[self.level];    
            child.rewind().with_color(color);            

        }

    }
    pub fn spin<T: From<HslColor>>(&mut self, callback: &mut dyn FnMut(T)) {
        for color in self { 
            callback(color.into()); 
        }
    }

    pub fn spin_vec<T: From<HslColor>>(&mut self) -> Vec<T> 
    where T: Into<HslColor> {

        let mut result = Vec::<T>::new();
        for color in self {
            result.push(color.into());
        }

        result
    }
        
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_float_eq::*;

    #[test]
    fn spin_cb_count_4x3() {
        let mut count = 0;
        KolorWheel::new(HslColor::new(0, 0, 0), 4)
            .fork(3)
            .spin(&mut|_color: HslColor| { count += 1 })
        ;
        assert_eq!(count, 12);
    }

    #[test]
    fn spin_cb_count_1x5() {
        let mut count = 0;
        KolorWheel::new(HslColor::new(0, 0, 0), 1)
            .fork(5)
            .spin(&mut|_color: HslColor| { count += 1 })
        ;
        assert_eq!(count, 5);
    }

    #[test]
    fn spin_cb_count_6x1() {
        let mut count = 0;
        KolorWheel::new(HslColor::new(0, 0, 0), 6)
            .fork(1)
            .spin(&mut|_color: HslColor| { count += 1 })
        ;
        assert_eq!(count, 6);
    }

    #[test]
    fn spin_cb_count_1x1() {
        let mut count = 0;
        KolorWheel::new(HslColor::new(0, 0, 0), 1)
            .fork(1)
            .spin(&mut|_color: HslColor| { count += 1 })
        ;
        assert_eq!(count, 1);
    }

    #[test]
    fn spin_cb_count_2x3x4x5() {
        let mut count = 0;
        KolorWheel::new(HslColor::new(0, 0, 0), 2)
            .fork(3)
            .fork(4)
            .fork(5)
            .spin(&mut|_color: HslColor| { count += 1 })
        ;
        assert_eq!(count, 120);
    }

    #[test]
    fn spin_cb_hue_abs_simple() {
        let mut result: Vec<HslColor> = Vec::new();
        let base = HslColor::new(0, 100, 50);
        KolorWheel::new(base, 3)
            .with_hue(SpinMode::Absolute(10))
            .spin(&mut|color: HslColor| { 
                result.push(color); 
            })
        ;
        assert_f32_near!(result[0].h, 0.0, 99999);
        assert_f32_near!(result[1].h, 5.0, 99999);
        assert_f32_near!(result[2].h, 10.0, 99999);
    }

    #[test]
    fn spin_vec_hue_abs_negative() {
        let base = HslColor::new(120, 100, 50);
        let result = KolorWheel::new(base, 3)
            .with_hue(SpinMode::Absolute(100))
            .spin_vec::<HslColor>()
        ;
        assert_f32_near!(result[0].h, 120.0, 99999);
        assert_f32_near!(result[1].h, 110.0, 99999);
        assert_f32_near!(result[2].h, 100.0, 99999);
    }

    #[test]
    fn spin_vec_fade_to_black() {
        let base = HslColor::new(180, 100, 50);
        let result = KolorWheel::new(base, 5)
            .with_macro(SpinMacro::FadeToBlack)
            .spin_vec::<HslColor>()
        ;
        assert!(result[3].s > 5.0);
        assert!(result[3].l > 2.5);
        assert_f32_near!(result[4].s, 0.0, 99999);
        assert_f32_near!(result[4].l, 0.0, 99999);
    }

    #[test]
    fn spin_vec_fade_to_white() {
        let base = HslColor::new(90, 80, 10);
        let result = KolorWheel::new(base, 5)
            .with_macro(SpinMacro::FadeToWhite)
            .spin_vec::<HslColor>()
        ;
        assert!(result[3].s > 5.0);
        assert!(result[3].l < 98.3);
        assert_f32_near!(result[4].s, 0.0, 99999);
        assert_f32_near!(result[4].l, 100.0, 99999);
    }

    #[test]
    fn spin_iter_fade_to_gray() {

        let base = HslColor::new(90, 80, 10);
        let mut kw = KolorWheel::new(base, 5);
        kw.with_macro(SpinMacro::FadeToGray(40));

        let mut index = 0;
        for color in kw {

            if index == 3 {
                assert!(color.s > 5.0);
                assert!(color.l < 98.3);
            }

            if index == 4 {
                assert_f32_near!(color.s, 0.0, 99999);
                assert_f32_near!(color.l, 40.0, 99999);
            }

            index += 1;
        }


    }

}
