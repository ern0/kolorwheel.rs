extern crate kolorwheel;
use kolorwheel::KolorWheel;
use kolorwheel::SpinMode;
use kolorwheel::hsl_color::HslColor;

use crate::App;
use crate::Panel;

pub struct HueRelUniv {
    include: bool,
    cols: u32,
    rows: u32,
    color: HslColor,
    hue: i32,
}

impl HueRelUniv {

    pub fn new(include: bool) -> Self {
        Self {
            include,
            cols: 3,
            rows: 2,
            color: HslColor::new(0, 100, 50),
            hue: 360,
        }
    }
}

impl Panel for HueRelUniv {
    
    fn get_source_script(&self) -> &str {
        return file!();
    }
  
    fn paint(&mut self, ui: &mut egui::Ui) -> (KolorWheel, u32, u32) {

        ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
            ui.label("Base color:");
            App::paint_hsl_sliders(ui, &mut self.color);
            ui.label(
                if self.include { "  Change hue relative, including target:" }
                else { "  Change hue relative, excluding target:" }
            );
            ui.add(
                egui::Slider::new(&mut self.hue, -360..=360)
                .orientation(egui::SliderOrientation::Vertical)
                .trailing_fill(true)
                .text("Hue+")
                .suffix("°")
            );

        });

        let mut kw = KolorWheel::new(self.color, (self.cols * self.rows) as usize);

        if self.include {
            kw.with_hue(SpinMode::RelativeIncl(self.hue));
        } else {
            kw.with_hue(SpinMode::RelativeExcl(self.hue));
        }

        (kw, self.cols, self.rows)
    }

}
