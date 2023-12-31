extern crate kolorwheel;
use kolorwheel::{ KolorWheel, SpinMode, SpinMacro, HslColor };

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
            color1: HslColor::new(270, 70, 70),
            color2: HslColor::new(270, 80, 30),
            count: 4,
            values: [ 0, -150, 120, -210, 95, -90, 325, -330 ],
        }
    }
}

impl Panel for HueOffsets {
    
    fn get_source_script(&self) -> &str {
        file!()
    }

    fn paint(&mut self, ui: &mut egui::Ui) -> (KolorWheel, u32, u32) {

        ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {

            let mut hue: i32 = self.color1.h as i32;
            ui.label("Hue:");
            ui.add(
                egui::Slider::new(&mut hue, 0..=359)
                .orientation(egui::SliderOrientation::Vertical)
                .trailing_fill(true)
            );
            self.color1.h = hue as f32;
            self.color2.h = hue as f32;

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

        let mut kw = KolorWheel::new(self.color1, self.rows as usize);
        kw.with_macro(SpinMacro::GradientColor(self.color2));
        kw.fork(self.count);
        kw.with_hue(SpinMode::Offset(&self.values[0 .. (self.cols as usize)]));

        (kw, self.cols, self.rows)
    }

}
