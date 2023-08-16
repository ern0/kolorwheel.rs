extern crate kolorwheel;
use kolorwheel::KolorWheel;
use kolorwheel::SpinMode;
use kolorwheel::FadeMode;
use kolorwheel::hsl_color::HslColor;

use egui;
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
            color: HslColor { h: 240, s: 80, l: 70 },
            sat_offsets: [60, 0, 0, 70, 0, 0],
            lit_offsets: [-40, 0, 0],
        }
    }
}

impl Panel for Palette2 {
    
    fn paint(&mut self, ui: &mut egui::Ui) -> (KolorWheel, u32, u32) {

        ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {

            ui.label("Base hue:");

            ui.add(
                egui::Slider::new(&mut self.color.h, 0..=359)
                .orientation(egui::SliderOrientation::Vertical)
                .trailing_fill(true)
                .text("Hue")
                .suffix("°")
            );
        });

        let kw = KolorWheel::new()
            .set_count(self.cols * self.rows)
            .set_hsl(self.color.h, self.color.s, self.color.l)
            .hue_reli(90)
            .sat_reli(-20)
            .sat_offs(&self.sat_offsets[0..6])
            .lit_offs(&self.lit_offsets[0..3])
        ;

        return (kw, self.cols, self.rows);
    }

}
