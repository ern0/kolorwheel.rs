extern crate kolorwheel;
use kolorwheel::KolorWheel;
use kolorwheel::SpinMacro;
use kolorwheel::hsl_color::HslColor;

use crate::App;
use crate::Panel;

pub struct Gradient {
    cols: u32,
    rows: u32,
    color1: HslColor,
    color2: HslColor,
}

impl Gradient {
    
    pub fn new() -> Self {
        Self {
            cols: 5,
            rows: 5,
            color1: HslColor::new(0, 100, 50),
            color2: HslColor::new(270, 70, 30),
        }
    }
}

impl Panel for Gradient {

    fn get_source_script(&self) -> &str {
        return file!();
    }

    fn paint(&mut self, ui: &mut egui::Ui) -> (KolorWheel, u32, u32) {

        ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
            ui.label("Base color:");
            App::paint_hsl_sliders(ui, &mut self.color1);
            ui.label("  Gradient to:");
            App::paint_hsl_sliders(ui, &mut self.color2);
        });

        let mut kw = KolorWheel::new(self.color1, (self.cols * self.rows) as usize);
        kw.with_macro(SpinMacro::GradientColor(self.color2));

        (kw, self.cols, self.rows)
    }
}
