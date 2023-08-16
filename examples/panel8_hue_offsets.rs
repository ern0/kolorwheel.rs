extern crate kolorwheel;
use kolorwheel::KolorWheel;
use kolorwheel::SpinMode;
use kolorwheel::FadeMode;
use kolorwheel::hsl_color::HslColor;

use egui;
use crate::Panel;

pub struct HueOffsets {
    cols: u32,
    rows: u32,
    color1: HslColor,
    color2: HslColor,
    count: usize,
    values: [i32; 8],
}

impl HueOffsets {
    
    pub fn new() -> Self {
        Self {
            cols: 8,
            rows: 6,
            color1: HslColor { h: 270, s: 70, l: 70 },
            color2: HslColor { h: 270, s: 80, l: 30 },
            count: 4,
            values: [ 0, -150, 120, -210, 95, -90, 325, -330 ],
        }
    }
}

impl Panel for HueOffsets {
    
    fn paint(&mut self, ui: &mut egui::Ui) -> (KolorWheel, u32, u32) {

        ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {

            ui.label("Count:");
            ui.add(
                egui::Slider::new(&mut self.count, 1..=8)
                .orientation(egui::SliderOrientation::Vertical)
                .trailing_fill(true)
            );

            ui.label("  Hue offsets:");

            for i in 0..self.count {
                ui.add(
                    egui::Slider::new(&mut self.values[i], -359..=359)
                    .orientation(egui::SliderOrientation::Vertical)
                    .trailing_fill(true)
                    .text("Hue+")
                    .suffix("°")
                );
            }

        });

        let kw = KolorWheel::new()
            .set_count(self.cols * self.rows)
            .set_hsl(self.color1.h, self.color1.s, self.color1.l)
            .hue_offs(&self.values[0 .. self.count])
            .gradient(KolorWheel::new().set_hsl(self.color2.h, self.color2.s, self.color2.l))
        ;

        return (kw, self.cols, self.rows);
    }

}
