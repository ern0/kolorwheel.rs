extern crate kolorwheel;
use kolorwheel::{ KolorWheel, SpinMode, HslColor };

use crate::App;
use crate::Panel;

pub struct SatLitRel {
    cols: u32,
    rows: u32,
    color: HslColor,
    sat: i32,
    lit: i32,
}

impl SatLitRel {
   
    pub fn new() -> Self {
        Self {
            cols: 8,
            rows: 6,
            color: HslColor::new(60, 70, 50),
            sat: -50,
            lit: -15,
        }
    }
}

impl Panel for SatLitRel {
    
    fn get_source_script(&self) -> &str {
        file!()
    }

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

        let mut kw = KolorWheel::new(self.color, (self.cols * self.rows) as usize);
        kw.with_saturation(SpinMode::RelativeIncl(self.sat));
        kw.with_lightness(SpinMode::RelativeIncl(self.lit));

        (kw, self.cols, self.rows)
    }

}
