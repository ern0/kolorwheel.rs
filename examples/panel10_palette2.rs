extern crate kolorwheel;
use kolorwheel::KolorWheel;
use kolorwheel::SpinMode;
use kolorwheel::hsl_color::HslColor;

use crate::Panel;

pub struct Palette2 {
    cols: u32,
    rows: u32,
    color: HslColor,
    sat_offsets: [i32; 6],
    lit_offsets: [i32; 3],
}

impl Palette2 {

    pub fn new() -> Self {
        Self {
            cols: 3,
            rows: 2,
            color: HslColor::new(240, 80, 70),
            sat_offsets: [60, 0, 0, 70, 0, 0],
            lit_offsets: [-40, 0, 0],
        }
    }
}

impl Panel for Palette2 {

    fn get_source_script(&self) -> &str {
        return file!();
    }

    fn paint(&mut self, ui: &mut egui::Ui) -> (KolorWheel, u32, u32) {

        ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {

            ui.label("Base hue:");

            let mut h = self.color.h as i32;
            ui.add(
                egui::Slider::new(&mut h, 0..=359)
                .orientation(egui::SliderOrientation::Vertical)
                .trailing_fill(true)
                .text("Hue")
                .suffix("°")
            );
            self.color.h = h as f32;
        });

        let mut kw = KolorWheel::new(self.color, (self.cols * self.rows) as usize);

        kw.with_hue(SpinMode::RelativeIncl(90));
        kw.with_saturation(SpinMode::RelativeIncl(-20));
        kw.with_saturation(SpinMode::Offset(&self.sat_offsets[0..6]));
        kw.with_lightness(SpinMode::Offset(&self.lit_offsets[0..3]));

        (kw, self.cols, self.rows)
    }

}
