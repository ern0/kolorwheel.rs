extern crate kolorwheel;
use kolorwheel::{ KolorWheel, SpinMode, HslColor };

use crate::Panel;

pub struct Palette1 {
    cols: u32,
    rows: u32,
    color: HslColor,
    hue_offsets: [i32; 5],
    lit_offsets: [i32; 5],
}

impl Palette1 {

    pub fn new() -> Self {
        Self {
            cols: 5,
            rows: 1,
            color: HslColor::new(20, 70, 60),
            hue_offsets: [0, 0, 0, 0, 120],
            lit_offsets: [0, 0, 0, 0, -60],
        }
    }
}

impl Panel for Palette1 {
    
    fn get_source_script(&self) -> &str {
        file!()
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
        kw.with_hue(SpinMode::RelativeIncl(75));
        kw.with_hue(SpinMode::Offset(&self.hue_offsets[0..5]));
        kw.with_lightness(SpinMode::RelativeIncl(30));
        kw.with_lightness(SpinMode::Offset(&self.lit_offsets[0..5]));

        (kw, self.cols, self.rows)
    }

}
