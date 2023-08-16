extern crate kolorwheel;
use kolorwheel::KolorWheel;
use kolorwheel::SpinMode;
use kolorwheel::FadeMode;
use kolorwheel::hsl_color::HslColor;

use egui;
use crate::App;
use crate::Panel;

pub struct SatAbs {
    cols: u32,
    rows: u32,
    color: HslColor,
    sat: i32,
}

impl SatAbs {
    
    pub fn new() -> Self {
        Self {
            cols: 4,
            rows: 4,
            color: HslColor { h: 180, s: 30, l: 50 },
            sat: 0,
        }
    }
}

impl Panel for SatAbs {

    fn paint(&mut self, ui: &mut egui::Ui) -> (KolorWheel, u32, u32) {

        ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
            ui.label("Base color:");
            App::paint_hsl_sliders(ui, &mut self.color);
            ui.label("  Change saturation to absolute:");
            ui.add(
                egui::Slider::new(&mut self.sat, 0..=100)
                .orientation(egui::SliderOrientation::Vertical)
                .trailing_fill(true)
                .text("Sat")
                .suffix("%")
            );
        });

        let kw = KolorWheel::new()
            .set_count(self.cols * self.rows)
            .set_hsl(self.color.h, self.color.s, self.color.l)
            .sat_abs((self.sat as u32).try_into().unwrap())
        ;

        return (kw, self.cols, self.rows);
    }

}
