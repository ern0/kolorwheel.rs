use crate::hsl_color::HslColor;
use crate::SpinMode;

pub(crate) struct Spinner {

    color: HslColor,    
    count: usize,
    counter: usize,  

    spin_calculated_hue: SpinCalculated,
    spin_calculated_saturation: SpinCalculated,
    spin_calculated_lightness: SpinCalculated,

    spin_stored_hue: SpinStored,
    spin_stored_saturation: SpinStored,
    spin_stored_lightness: SpinStored,

}

enum SpinCalculated {
    Inactive,
    Active(f32),
}

enum SpinStored {
    Inactive,
    Active(Vec<i32>),
}

impl<'sp> Spinner {

    pub(crate) fn new<T>(color: T, count: usize) -> Self 
    where T: Into<HslColor> {
        Self {
            color: color.into(),
            count,
            counter: 0,

            spin_calculated_hue: SpinCalculated::Inactive,
            spin_calculated_saturation: SpinCalculated::Inactive,
            spin_calculated_lightness: SpinCalculated::Inactive,

            spin_stored_hue: SpinStored::Inactive,
            spin_stored_saturation: SpinStored::Inactive,
            spin_stored_lightness: SpinStored::Inactive,
        }
    }

    pub(crate) fn color(&self) ->  HslColor {
        self.color
    }

    pub(crate) fn rewind(&mut self) -> &mut Self {
        self.counter = 0;
        self
    }

    pub(crate) fn with_color(&mut self, color: HslColor) -> &mut Self {
        self.color = color;
        self
    }

    pub(crate) fn with_hue(&mut self, spin_mode: SpinMode<'sp>) {

        match spin_mode {
            SpinMode::Still => (),
            SpinMode::Offset(_) => {
                self.spin_stored_hue = Self::store_spin_values(spin_mode);
            },
            _ => {
                self.spin_calculated_hue = Self::calc_spin_value(spin_mode, self.color.h, self.count);
            },
        }        

    }


    pub(crate) fn with_saturation(&mut self, spin_mode: SpinMode<'sp>) {

        match spin_mode {
            SpinMode::Still => (),
            SpinMode::Offset(_) => {
                self.spin_stored_saturation = Self::store_spin_values(spin_mode);
            },
            _ => {
                self.spin_calculated_saturation = Self::calc_spin_value(spin_mode, self.color.s, self.count);
            },
        }        

    }

    pub(crate) fn with_lightness(&mut self, spin_mode: SpinMode<'sp>) {
        
        match spin_mode {
            SpinMode::Still => (),
            SpinMode::Offset(_) => {
                self.spin_stored_lightness = Self::store_spin_values(spin_mode);
            },
            _ => {
                self.spin_calculated_lightness = Self::calc_spin_value(spin_mode, self.color.l, self.count);
            },
        }        

    }

    fn calc_spin_value(spin_mode: SpinMode<'sp>, base_value: f32, count: usize) -> SpinCalculated {

        match spin_mode { 

            SpinMode::Absolute(abs_target) => {
                let abs_target = abs_target as f32;
                let rel_target = abs_target - base_value;
                let step = (count - 1) as f32;
                let inc = rel_target / step;

                SpinCalculated::Active(inc)
            },

            SpinMode::RelativeIncl(rel_target) => {
                let rel_target = rel_target as f32;
                let step = (count - 1) as f32;
                let inc = rel_target / step;

                SpinCalculated::Active(inc)
            },

            SpinMode::RelativeExcl(rel_target) => {
                let rel_target = rel_target as f32;
                let step = count as f32;
                let inc = rel_target / step;

                SpinCalculated::Active(inc)
            },

            _ => SpinCalculated::Inactive,
        }
    }

    fn store_spin_values(spin_mode: SpinMode<'sp>) -> SpinStored {

        match spin_mode { 
            SpinMode::Offset(values) => SpinStored::Active(Vec::from(values)),
            _ => SpinStored::Inactive,
        }
    }

    pub(crate) fn spin_finished(&self) -> bool {
        self.counter >= self.count
    }

    pub(crate) fn spin_next(&mut self) ->  HslColor {

        if self.counter > 0 {
            self.spin_calculated_hsl();
        }
        let mut offseted_color = self.spin_stored_hsl();
        offseted_color.normalize();

        self.counter += 1;
        offseted_color
    }

    fn spin_calculated_hsl(&mut self) {

        Self::spin_calculated_channel(&mut self.color.h, &self.spin_calculated_hue);
        Self::spin_calculated_channel(&mut self.color.s, &self.spin_calculated_saturation);
        Self::spin_calculated_channel(&mut self.color.l, &self.spin_calculated_lightness);
    }

    fn spin_calculated_channel(channel_value: &mut f32, channel_spin: &SpinCalculated) {

        if let SpinCalculated::Active(channel_inc) = channel_spin {
            *channel_value += channel_inc;
        }
    }

    fn spin_stored_hsl(&self) -> HslColor {

        let h = Self::spin_stored_channel(self.color.h, &self.spin_stored_hue, self.counter);
        let s = Self::spin_stored_channel(self.color.s, &self.spin_stored_saturation, self.counter);
        let l = Self::spin_stored_channel(self.color.l, &self.spin_stored_lightness, self.counter);

        HslColor::from((h, s, l,))
    }

    fn spin_stored_channel(channel_value: f32, channel_spin: &SpinStored, counter: usize) -> f32 {

        let mut channel_result = channel_value;

        if let SpinStored::Active(offsets) = channel_spin {
            let index = counter % offsets.len();
            channel_result += offsets[index] as f32;
        }

        channel_result
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use all_asserts::*;
    use assert_float_eq::*;

    #[test]
    fn spinner_hue_abs_simple() {

        let color = HslColor::new(0, 100, 50);
        let mut spinner = Spinner::new(color, 3);
        spinner.with_hue(SpinMode::Absolute(2));

        let result = spinner.spin_next();
        assert_f32_near!(result.h, 0.0, 99999);
        let finished = spinner.spin_finished();
        assert_false!(finished);

        let result = spinner.spin_next();
        assert_f32_near!(result.h, 1.0, 99999);
        let finished = spinner.spin_finished();
        assert_false!(finished);

        let result = spinner.spin_next();
        assert_f32_near!(result.h, 2.0, 99999);
        let finished = spinner.spin_finished();
        assert_true!(finished);
    }

    #[test]
    fn spinner_sat_rel_incl() {

        let color = HslColor::new(0, 20, 50);
        let mut spinner = Spinner::new(color, 3);
        spinner.with_saturation(SpinMode::RelativeIncl(10));

        let result = spinner.spin_next();
        assert_f32_near!(result.s, 20.0, 99999);

        let result = spinner.spin_next();
        assert_f32_near!(result.s, 25.0, 99999);

        let result = spinner.spin_next();
        assert_f32_near!(result.s, 30.0, 99999);
    }

    #[test]
    fn spinner_lit_rel_excl() {

        let color = HslColor::new(0, 20, 50);
        let mut spinner = Spinner::new(color, 3);
        spinner.with_lightness(SpinMode::RelativeExcl(10));

        let result = spinner.spin_next();
        assert_f32_near!(result.l, 50.0, 99999);

        let result = spinner.spin_next();
        assert_f32_near!(result.l, 53.333, 99999);

        let result = spinner.spin_next();
        assert_f32_near!(result.l, 56.667, 99999);
    }

    #[test]
    fn spinner_circular_overflow() {

        let color = HslColor::new(0, 100, 50);
        let mut spinner = Spinner::new(color, 5);
        spinner.with_hue(SpinMode::RelativeIncl(400));

        let result = spinner.spin_next();
        assert_f32_near!(result.h, 0.0, 99999);

        let result = spinner.spin_next();
        assert_f32_near!(result.h, 100.0, 99999);
        
        let result = spinner.spin_next();
        assert_f32_near!(result.h, 200.0, 99999);

        let result = spinner.spin_next();
        assert_f32_near!(result.h, 300.0, 99999);

        let result = spinner.spin_next();
        assert_f32_near!(result.h, 400.0 - 360.0, 99999);
    }

    #[test]
    fn spinner_circular_underflow_hue() {

        let color = HslColor::new(0, 100, 50);
        let mut spinner = Spinner::new(color, 5);
        spinner.with_hue(SpinMode::RelativeIncl(-400));

        let result = spinner.spin_next();
        assert_f32_near!(result.h, 0.0, 99999);

        let result = spinner.spin_next();
        assert_f32_near!(result.h, -100.0 + 360.0, 99999);        
    }

    #[test]
    fn spinner_percent_overflow_sat() {

        let color = HslColor::new(0, 90, 50);
        let mut spinner = Spinner::new(color, 5);
        spinner.with_saturation(SpinMode::RelativeIncl(100));

        let result = spinner.spin_next();
        assert_f32_near!(result.s, 90.0, 99999);        

        let result = spinner.spin_next();
        assert_f32_near!(result.s, 100.0, 99999);        

        let result = spinner.spin_next();
        assert_f32_near!(result.s, 100.0, 99999);        
    }

    #[test]
    fn spinner_percent_underflow_lit() {

        let color = HslColor::new(0, 100, 10);
        let mut spinner = Spinner::new(color, 5);
        spinner.with_lightness(SpinMode::RelativeExcl(-200));

        let result = spinner.spin_next();
        assert_f32_near!(result.l, 10.0, 99999);

        let result = spinner.spin_next();
        assert_f32_near!(result.l, 0.0, 99999);        

        let result = spinner.spin_next();
        assert_f32_near!(result.l, 0.0, 99999);        
    }

    #[test]
    fn spinner_stored_normal() {

        let color = HslColor::new(0, 100, 50);
        let mut spinner = Spinner::new(color, 5);
        spinner.with_lightness(SpinMode::Offset(&[-10, 10]));

        let result = spinner.spin_next();
        assert_f32_near!(result.l, 40.0, 99999);        

        let result = spinner.spin_next();
        assert_f32_near!(result.l, 60.0, 99999);        

        let result = spinner.spin_next();
        assert_f32_near!(result.l, 40.0, 99999);        
    }

    #[test]
    fn spinner_stored_overflow() {

        let color = HslColor::new(0, 100, 95);
        let mut spinner = Spinner::new(color, 5);
        spinner.with_lightness(SpinMode::Offset(&[-10, 10]));

        let result = spinner.spin_next();
        assert_f32_near!(result.l, 85.0, 99999);        

        let result = spinner.spin_next();
        assert_f32_near!(result.l, 100.0, 99999);        

        let result = spinner.spin_next();
        assert_f32_near!(result.l, 85.0, 99999);        
    }

}
