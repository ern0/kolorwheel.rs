extern crate kolorwheel;
use kolorwheel::KolorWheel;
use kolorwheel::SpinMode;
use kolorwheel::hsl_color::HslColor;

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
            color: HslColor::new(180, 30, 50),
            sat: 0,
        }
    }
}

impl Panel for SatAbs {

    fn get_source_script(&self) -> &str {
        return file!();
    }

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

        let mut kw = KolorWheel::new(self.color, (self.cols * self.rows) as usize);
        kw.with_saturation(SpinMode::Absolute(self.sat));

        (kw, self.cols, self.rows)
    }

}
