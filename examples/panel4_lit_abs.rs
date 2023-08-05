extern crate kolorwheel;
use kolorwheel::KolorWheel;

use egui;
use crate::hsl::Hsl;
use crate::App;
use crate::Panel;

pub struct LitAbs {
    cols: u32,
    rows: u32,
    color: Hsl,
    lit: i32,
}

impl LitAbs {
    
    pub fn new() -> Self {
        Self {
            cols: 4,
            rows: 4,
            color: Hsl { h: 140, s: 70, l: 60 },
            lit: 40,
        }
    }
}

impl Panel for LitAbs {
    
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

        let kw = KolorWheel::new()
            .set_count(self.cols * self.rows)
            .set_hsl(self.color.h, self.color.s, self.color.l)
            .lit_abs((self.lit as u32).try_into().unwrap())
        ;

        return (kw, self.cols, self.rows);
    }

}
