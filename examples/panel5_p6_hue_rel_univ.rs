extern crate kolorwheel;
use kolorwheel::KolorWheel;

use egui;
use crate::hsl::Hsl;
use crate::App;
use crate::Panel;

pub struct HueRelUniv {
    include: bool,
    cols: u32,
    rows: u32,
    color: Hsl,
    hue: i32,
}

impl HueRelUniv {
    
    pub fn new(include: bool) -> Self {
        Self {
            include,
            cols: 3,
            rows: 2,
            color: Hsl { h: 0, s: 100, l: 50 },
            hue: 360,
        }
    }
}

impl Panel for HueRelUniv {
    
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

        let mut kw = KolorWheel::new()
            .set_count(self.cols * self.rows)
            .set_hsl(self.color.h, self.color.s, self.color.l)
        ;

        if self.include {
            kw = kw.hue_reli((self.hue as i32).try_into().unwrap());
        } else {
            kw = kw.hue_relx((self.hue as i32).try_into().unwrap());
        }

        return (kw, self.cols, self.rows);
    }

}
