extern crate kolorwheel;
use kolorwheel::{ KolorWheel, SpinMode, HslColor };

use crate::App;
use crate::Panel;

pub struct LitAbs {
    cols: u32,
    rows: u32,
    color: HslColor,
    lit: i32,
}

impl LitAbs {

    pub fn new() -> Self {
        Self {
            cols: 4,
            rows: 4,
            color: HslColor::new(140, 70, 60),
            lit: 40,
        }
    }
}

impl Panel for LitAbs {
    
    fn get_source_script(&self) -> &str {
        return file!();
    }

    fn paint(&mut self, ui: &mut egui::Ui) -> (KolorWheel, u32, u32) {

        ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
            ui.label("Base color:");
            App::paint_hsl_sliders(ui, &mut self.color);
            ui.label("  Change lightness to absolute:");
            ui.add(
                egui::Slider::new(&mut self.lit, 0..=100)
                .orientation(egui::SliderOrientation::Vertical)
                .trailing_fill(true)
                .text("Lit")
                .suffix("%")
            );
        });

        let mut kw = KolorWheel::new(self.color, (self.cols * self.rows) as usize);
        kw.with_lightness(SpinMode::Absolute(self.lit));

        (kw, self.cols, self.rows)
    }

}
