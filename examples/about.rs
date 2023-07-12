#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused_variables)]

use eframe::egui;
extern crate kolorwheel;

fn main() -> Result<(), eframe::Error> {

    let initial_size_vec2 = egui::vec2(320.0, 256.0);
    let columns = 8;
    let padding = 0.2;

    let options = eframe::NativeOptions {
        initial_window_size: Some(initial_size_vec2),
        ..Default::default()
    };

    eframe::run_simple_native("KolorWheel.rs", options, move |ctx, _frame| {

        egui::CentralPanel::default().show(ctx, |ui| {

        let width = ui.available_width();
        let height = ui.available_height();

        let mut app_window = AppWindow::new(ui, width, height, columns, padding);
            app_window.show_panel();
        });
    })
}

struct AppWindow<'u> {
    ui: &'u mut egui::Ui,
    width: f32,
    height: f32,
    columns: u32,
    padding: f32,
}

impl AppWindow<'_> {

    pub fn new(ui: &mut egui::Ui, width: f32, height: f32, columns: u32, padding: f32) -> AppWindow {
        AppWindow { ui, width, height, columns, padding }
    }

    fn show_panel(&mut self) {

        self.ui.label(format!("{}x{}", self.width, self.height));

        let rect = egui::Rect {
            min: egui::Pos2{ x: 50.0, y: 50.0 },
            max: egui::Pos2{ x: 100.0, y: 100.0 },
        };

        let fill = egui::Color32::BLUE;
    }

    fn show_box(&mut self, rect: egui::Rect, fill: egui::Color32) {

        // let (_, painter) = self.ui.allocate_painter(
        //     egui::Vec2::new(self.window_width, self.window_height),
        //     egui::Sense::hover(),
        // );

        //let rounding = egui::Rounding {
            //nw: self.box_rounding, ne: self.box_rounding,
            //sw: self.box_rounding, se: self.box_rounding,
        //};

        //let stroke = egui::epaint::Stroke{
            //width: self.box_rounding,
            //color: fill,
        //};

        //let rect_shape = egui::epaint::RectShape { rect, rounding, fill, stroke };
        //let rectangle = egui::Shape::Rect(rect_shape);
        //painter.add(rectangle);

    }
}
