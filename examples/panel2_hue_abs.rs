extern crate kolorwheel;
use kolorwheel::KolorWheel;
use kolorwheel::SpinMode;
use kolorwheel::hsl_color::HslColor;

use crate::App;
use crate::Panel;

pub struct HueAbs {
    cols: u32,
    rows: u32,
    color: HslColor,
    hue: i32,
}

impl HueAbs {
    
    pub fn new() -> Self {
        Self {
            cols: 4,
            rows: 4,
            color: HslColor::new(0, 100, 50),
            hue: 120,
        }
    }
}

impl Panel for HueAbs {

    fn get_source_script(&self) -> &str {
        return file!();
    }

    fn paint(&mut self, ui: &mut egui::Ui) -> (KolorWheel, u32, u32) {

        ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
            ui.label("Base color:");
            App::paint_hsl_sliders(ui, &mut self.color);
            ui.label("  Change hue to absolute:");
            ui.add(
                egui::Slider::new(&mut self.hue, -359..=719)
                .orientation(egui::SliderOrientation::Vertical)
                .trailing_fill(true)
                .text("Hue")
                .suffix("°")
            );
        });

        let mut kw = KolorWheel::new(self.color, (self.cols * self.rows) as usize);
        kw.with_hue(SpinMode::Absolute(self.hue));

        (kw, self.cols, self.rows)
    }

}
