extern crate kolorwheel;
use kolorwheel::KolorWheel;

use egui;
use crate::hsl::Hsl;
use crate::Panel;

pub struct Palette1 {
    cols: u32,
    rows: u32,
    color: Hsl,
    hue_offsets: [i32; 5],
    lit_offsets: [i32; 5],
}

impl Palette1 {
    
    pub fn new() -> Self {
        Self {
            cols: 5,
            rows: 1,
            color: Hsl { h: 20, s: 70, l: 50 },
            hue_offsets: [0, 0, 0, 0, 120],
            lit_offsets: [0, 0, 0, 0, -40],
        }
    }
}

impl Panel for Palette1 {
    
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
            .hue_reli(75)
            .hue_offs(&self.hue_offsets[0..5])
            .lit_reli(30)
            .lit_offs(&self.lit_offsets[0..5])
        ;

        return (kw, self.cols, self.rows);
    }

}
