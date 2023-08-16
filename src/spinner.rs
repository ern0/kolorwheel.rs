use crate::hsl_color::HslColor;
use crate::SpinMode;

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
    Calculated(f32),
    Stored(&'sp [i32]),
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
            self.count,
        );

        self.spin_saturation = Self::recalculate_channel(
            self.color.s, 
            &self.spin_mode_saturation,
            self.count,
        );

        self.spin_lightness = Self::recalculate_channel(
            self.color.l, 
            &self.spin_mode_lightness,
            self.count,
        );

    }
    
    fn recalculate_channel(base: f32, spin_mode: &SpinMode<'i>, count: usize) -> Spin<'i> {

        match spin_mode {

            SpinMode::Unchanged => {
                Spin::Still
            },

            SpinMode::Absolute(abs_target) => {
                let abs_target = *abs_target as f32;
                let rel_target = abs_target - base;
                let step = (count - 1) as f32;
                let inc = rel_target / step;

                Spin::Calculated(inc)
            },

            SpinMode::RelativeIncl(rel_target) => {
                let rel_target = *rel_target as f32;
                let step = (count - 1) as f32;
                let inc = rel_target / step;

                Spin::Calculated(inc)
            },

            SpinMode::RelativeExcl(rel_target) => {
                let rel_target = *rel_target as f32;
                let step = count as f32;
                let inc = rel_target / step;

                Spin::Calculated(inc)
            },

            SpinMode::Offset(offsets) => {
                Spin::Stored(offsets)
            },

        }
    }

    pub(crate) fn spin_finished(&self) -> bool {
        self.counter >= self.count
    }

    pub(crate) fn spin_next(&mut self) ->  HslColor {

        if self.recalc_request { self.recalculate(); }

        if self.counter > 0 {
            self.spin_calculated_hsl();
        }
        let mut offseted_color = self.spin_stored_hsl();
        offseted_color.normalize();

        self.counter += 1;
        offseted_color
    }

    fn spin_calculated_hsl(&mut self) {

        Self::spin_calculated_channel(&mut self.color.h, &self.spin_hue);
        Self::spin_calculated_channel(&mut self.color.s, &self.spin_saturation);
        Self::spin_calculated_channel(&mut self.color.l, &self.spin_lightness);
    }

    fn spin_calculated_channel(channel_value: &mut f32, channel_spin: &Spin) {

        if let Spin::Calculated(channel_inc) = channel_spin {
            *channel_value += channel_inc;
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

        if let Spin::Stored(offsets) = channel_spin {
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
