extern crate kolorwheel;
use kolorwheel::KolorWheel;

use egui;
use crate::hsl::Hsl;
use crate::App;

pub struct P1Gradient {
    pub cols: u32,
    pub rows: u32,
    color1: Hsl,
    color2: Hsl,
}

impl P1Gradient {
	
	pub fn new() -> Self {
		Self {
            cols: 5,
            rows: 5,
			color1: Hsl { h: 0, s: 100, l: 50 },
			color2: Hsl { h: 270, s: 70, l: 30 },
		}
	}

	pub fn paint(&mut self, ui: &mut egui::Ui) -> (KolorWheel, u32, u32) {

		ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
            ui.label("Base color:");
            App::paint_hsl_sliders(ui, &mut self.color1);
            ui.label("  Gradient to:");
            App::paint_hsl_sliders(ui, &mut self.color2);
		});

		let kw = KolorWheel::new()
            .set_count(self.cols * self.rows)
            .set_hsl(self.color1.h, self.color1.s, self.color1.l)
            .gradient(KolorWheel::new().set_hsl(self.color2.h, self.color2.s, self.color2.l))
        ;

        return (kw, self.cols, self.rows);
	}

}
