#![allow(unused)]

use std::rc::Rc;

mod hsl_color;
mod rgb_color;
mod convert_hsl_to_rgb;
mod convert_rgb_to_hsl;

use hsl_color::HslColor;
use rgb_color::RgbColor;

#[derive(Clone)]
pub struct KolorWheel<'sp> {
    color: HslColor,    
    count: usize,
    skip_first: bool,
    chain_child: Option<Box<KolorWheel<'sp>>>,
    spin_hue: Spin<'sp>,
    spin_saturation: Spin<'sp>,
    spin_lightness: Spin<'sp>,
}

#[derive(Clone)]
pub enum Spin<'sl> {
    Unchanged,
    Absolute(i32),
    Relative(i32),
    Offset(&'sl [i32]),
    Stored(Vec<i32>),
}

impl<'a> KolorWheel<'a> {

    pub fn new<T>(color: T, count: usize) -> Self 
    where T: Into<HslColor> {
        Self {
            color: color.into(),
            count,
            skip_first: false,
            chain_child: None,
            spin_hue: Spin::Unchanged,
            spin_saturation: Spin::Unchanged,
            spin_lightness: Spin::Unchanged,
        }
    }

    pub fn with_hsl<T>(&mut self, target: T) -> &mut Self 
    where T: Into<HslColor> {       
        let hsl: HslColor = target.into();
        self.with_hue(Spin::Absolute(hsl.h as i32));
        self.with_saturation(Spin::Absolute(hsl.s as i32));
        self.with_lightness(Spin::Absolute(hsl.l as i32));
        self
    }

    pub fn with_hue(&mut self, spin: Spin<'a>) -> &mut Self {
        self.spin_hue = spin;
        self
    }

    pub fn with_saturation(&mut self, spin: Spin<'a>) -> &mut Self {
        self.spin_saturation = spin;
        self
    }

    pub fn with_lightness(&mut self, spin: Spin<'a>) -> &mut Self {
        self.spin_lightness = spin;
        self
    }

    pub fn skip_first(&mut self) -> &mut Self {
        self.skip_first = true;
        self
    }

    pub fn spin<T: From<HslColor>>(&mut self, callback: &dyn Fn(T)) {

        loop {
            let result = self.next();
            match result {
                Some(color) => callback(color.into()),
                None => break,
            }
        }

    }

    fn next(&mut self) -> Option<HslColor> {

        return None;
        
    }
        
        // for i in 0..self.count {     
        //     self.color.h = i as f32; 
        //     self.color.s = self.count as f32;
            
        //     let result: T = self.color.into();

        //     self.color.l += 1.0;

        //     if self.skip_first && i == 0 { continue };
        // }

    pub fn chain(&mut self, count: usize) -> &mut Self {

        let mut child = self.clone();
        child.count = count;        
        self.chain_child = Some(Box::new(child));

        self
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
            .with_hsl(HslColor::new(0, 100, 100))
            // .with_hue(Spin::Absolute(90))
            // .with_saturation(Spin::RelativeIncl(-10))
            // .with_lightness(Spin::Offset(&[0, 10]))
            // .skip_first()
            .chain(2)
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
