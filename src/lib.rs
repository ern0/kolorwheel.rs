#![allow(unused)]

mod hsl_color;
mod rgb_color;
mod convert_hsl_to_rgb;
mod convert_rgb_to_hsl;
mod spinner;

use hsl_color::HslColor;
use rgb_color::RgbColor;
use crate::spinner::{Spinner};

pub struct KolorWheel<'kw> {
    index: usize,
    spinner_vec: Vec<Spinner<'kw>>,
}

#[derive(Clone)]
pub enum SpinMode<'m> {
    Unchanged,
    Absolute(i32),
    RelativeIncl(i32),
    RelativeExcl(i32),
    Offset(&'m [i32]),
}

pub enum FadeMode {
    Color(HslColor),
    Gray(i32),
    Black,
    White,
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

    fn actual_spinner(&mut self) -> &mut Spinner<'kw> {
        &mut self.spinner_vec[self.index]
    }

    pub fn color<T>(&mut self, target: T) -> &mut Self 
    where T: Into<HslColor> {       
        self.actual_spinner().with_color(target.into());
        self
    }

    pub fn with_hue(&mut self, spin_mode: SpinMode<'kw>) -> &mut KolorWheel<'kw> {
        self.actual_spinner().with_hue(spin_mode);
        self
    }

    pub fn with_saturation(&mut self, spin_mode: SpinMode<'kw>) -> &mut KolorWheel<'kw> {
        self.actual_spinner().with_saturation(spin_mode);
        self
    }

    pub fn with_lightness(&mut self, spin_mode: SpinMode<'kw>) -> &mut Self {
        self.actual_spinner().with_lightness(spin_mode);
        self
    }

    pub fn fade(&mut self, fade_mode: FadeMode) -> &mut Self {

        let spinner = self.actual_spinner();

        match fade_mode {
            FadeMode::Color(hsl_color) => {
                self.with_hue(SpinMode::Absolute(hsl_color.h as i32));
                self.with_saturation(SpinMode::Absolute(hsl_color.s as i32));
                self.with_lightness(SpinMode::Absolute(hsl_color.l as i32));
            },
            FadeMode::Gray(percent) => {
                self.with_saturation(SpinMode::Absolute(0));
                self.with_lightness(SpinMode::Absolute(percent));
            },
            FadeMode::Black => {
                self.with_saturation(SpinMode::Absolute(0));
                self.with_lightness(SpinMode::Absolute(0));
            },
            FadeMode::White => {
                self.with_saturation(SpinMode::Absolute(0));
                self.with_lightness(SpinMode::Absolute(100));
            },
        };

        self
    }    

    pub fn fork(&mut self, count: usize) -> &mut Self {

        let color = self.actual_spinner().color();

        let mut spinner = Spinner::new(color, count);
        self.spinner_vec.push(spinner);
        self.index += 1;

        self
    }

    pub fn spin<T: From<HslColor>>(&mut self, callback: &mut dyn FnMut(T)) {

        let mut level = 0;
        let top_level = self.spinner_vec.len() - 1;
        self.spinner_vec[0].rewind();

        loop {

            let spinner = &mut self.spinner_vec[level];

            if spinner.spin_finished() {
                if level == 0 { return; }
                level -= 1;
                continue;
            }
                
            let color = spinner.spin_next();

            if level == top_level {  // render only top level
                callback(color.into());
                continue;
            }

            level += 1;
            let mut child = &mut self.spinner_vec[level];    
            child.rewind().with_color(color);                    

        } // loop
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_float_eq::*;

    #[test]
    fn spin_count_4x3() {
        let mut count = 0;
        let kw = KolorWheel::new(HslColor::new(0, 0, 0), 4)
            .fork(3)
            .spin(&mut|res: HslColor| { count += 1 })
        ;
        assert_eq!(count, 12);
    }

    #[test]
    fn spin_count_1x5() {
        let mut count = 0;
        let kw = KolorWheel::new(HslColor::new(0, 0, 0), 1)
            .fork(5)
            .spin(&mut|res: HslColor| { count += 1 })
        ;
        assert_eq!(count, 5);
    }

    #[test]
    fn spin_count_6x1() {
        let mut count = 0;
        let kw = KolorWheel::new(HslColor::new(0, 0, 0), 6)
            .fork(1)
            .spin(&mut|res: HslColor| { count += 1 })
        ;
        assert_eq!(count, 6);
    }

    #[test]
    fn spin_count_1x1() {
        let mut count = 0;
        let kw = KolorWheel::new(HslColor::new(0, 0, 0), 1)
            .fork(1)
            .spin(&mut|res: HslColor| { count += 1 })
        ;
        assert_eq!(count, 1);
    }

    #[test]
    fn spin_count_2x3x4x5() {
        let mut count = 0;
        let kw = KolorWheel::new(HslColor::new(0, 0, 0), 2)
            .fork(3)
            .fork(4)
            .fork(5)
            .spin(&mut|res: HslColor| { count += 1 })
        ;
        assert_eq!(count, 120);
    }
}

    // #[test]
    // fn tst() {

    //     println!(">>>>>>>>>>>>>>>>>>>>");

    //     let kw = KolorWheel::new(HslColor::new(0, 100, 50), 4)
    //         .fade(FadeMode::Color(HslColor::new(0, 100, 100)))
    //         .fade(FadeMode::Gray(50))
    //         .fade(FadeMode::Black)
    //         .fade(FadeMode::White)
    //         .with_hue(SpinMode::Absolute(90))
    //         .with_saturation(SpinMode::RelativeIncl(-10))
    //         .with_lightness(SpinMode::Offset(&[0, 10]))
    //         .fork(3)
    //         .spin(&|res: HslColor| {
    //             println!("-------------{:?}", res);
    //         })
    //     ;
    //     println!("<<<<<<<<<<<<<<<<<<<<");

    // }


