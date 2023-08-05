extern crate kolorwheel;
use kolorwheel::KolorWheel;

use egui;
use crate::hsl::Hsl;
use crate::App;
use crate::Panel;

pub struct SatLitRel {
    cols: u32,
    rows: u32,
    color: Hsl,
    sat: i32,
    lit: i32,
}

impl SatLitRel {
    
    pub fn new() -> Self {
        Self {
            cols: 8,
            rows: 6,
            color: Hsl { h: 60, s: 70, l: 50 },
            sat: -50,
            lit: -15,
        }
    }
}

impl Panel for SatLitRel {
    
    fn paint(&mut self, ui: &mut egui::Ui) -> (KolorWheel, u32, u32) {

        ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
            ui.label("Base color:");
            App::paint_hsl_sliders(ui, &mut self.color);
            ui.label("  Change saturation and lightness:");
            ui.add(
                egui::Slider::new(&mut self.sat, -100..=100)
                .orientation(egui::SliderOrientation::Vertical)
                .trailing_fill(true)
                .text("Sat+")
                .suffix("%")
            );
            ui.add(
                egui::Slider::new(&mut self.lit, -100..=100)
                .orientation(egui::SliderOrientation::Vertical)
                .trailing_fill(true)
                .text("Lit+")
                .suffix("%")
            );
        });

        let kw = KolorWheel::new()
            .set_count(self.cols * self.rows)
            .set_hsl(self.color.h, self.color.s, self.color.l)
            .sat_reli(self.sat)
            .lit_reli(self.lit)
        ;

        return (kw, self.cols, self.rows);
    }

}
