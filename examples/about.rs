#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused_variables)]

use eframe::egui;
extern crate kolorwheel;

fn main() -> Result<(), eframe::Error> {

    let window_width = 320.0;
    let window_height = 240.0;

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(window_width, window_height)),
        ..Default::default()
    };

    eframe::run_simple_native("KolorWheel.rs", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut app = App::new(ui);
            app.show_box();
        });
    })
}

struct App<'u> {
    ui: &'u mut egui::Ui,
}

impl App<'_> {

    pub fn new(ui: &mut egui::Ui) -> App {
        App { ui, }
    }

    fn show_box(&mut self) {

        let (_, painter) = self.ui.allocate_painter(
            egui::Vec2::new(
                self.ui.available_width(), 
                self.ui.available_height(),
            ),
            egui::Sense::hover(),
        );

        let rect = egui::Rect {
            min: egui::Pos2{ x: 50.0, y: 50.0 },
            max: egui::Pos2{ x: 100.0, y: 100.0 },
        };

        let uniform_rounding = 4.0;
        let rounding = egui::Rounding {
            nw: uniform_rounding, 
            ne: uniform_rounding,
            sw: uniform_rounding,
            se: uniform_rounding,
        };

        let fill = egui::Color32::BLUE;

        let stroke = egui::epaint::Stroke{
            width: uniform_rounding,
            color: fill,
        };

        let rect_shape = egui::epaint::RectShape { rect, rounding, fill, stroke };
        let rectangle = egui::Shape::Rect(rect_shape);
        painter.add(rectangle);

    }
}
