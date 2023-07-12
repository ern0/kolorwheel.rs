#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused_variables)]

use eframe::egui;
extern crate kolorwheel;

fn main() -> Result<(), eframe::Error> {

    let window_width = 320.0;
    let window_height = 240.0;
    let columns = 8;
    let padding = 0.2;

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(window_width, window_height)),
        ..Default::default()
    };

    eframe::run_simple_native("KolorWheel.rs", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut app = App::new(ui, columns, padding);
            app.show_panel();
        });
    })
}

struct App<'u> {
    ui: &'u mut egui::Ui,
    columns: u32,
    padding: f32,
    box_rounding: f32,
    window_width: f32,
    window_height: f32,
}

impl App<'_> {

    pub fn new(ui: &mut egui::Ui, columns: u32, padding: f32) -> App {
        App { 
            ui, 
            columns, 
            padding,
            box_rounding: 0.0,
            window_width: 0.0,
            window_height: 0.0,
        }
    }

    fn show_panel(&mut self) {

        self.window_width = self.ui.available_width();
        self.window_height = self.ui.available_height();
        self.box_rounding = self.window_width / 80.0;

        ///let column = self.columns % index;

        let rect = egui::Rect {
            min: egui::Pos2{ x: 50.0, y: 50.0 },
            max: egui::Pos2{ x: 100.0, y: 100.0 },
        };

        let fill = egui::Color32::BLUE;
    }

    fn show_box(&mut self, rect: egui::Rect, fill: egui::Color32) {

        let (_, painter) = self.ui.allocate_painter(
            egui::Vec2::new(self.window_width, self.window_height),
            egui::Sense::hover(),
        );

        let rounding = egui::Rounding {
            nw: self.box_rounding, ne: self.box_rounding,
            sw: self.box_rounding, se: self.box_rounding,
        };

        let stroke = egui::epaint::Stroke{
            width: self.box_rounding,
            color: fill,
        };

        let rect_shape = egui::epaint::RectShape { rect, rounding, fill, stroke };
        let rectangle = egui::Shape::Rect(rect_shape);
        painter.add(rectangle);

    }
}
