#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused_variables)]

use eframe::egui;
extern crate kolorwheel;

fn main() -> Result<(), eframe::Error> {

    let size_some_vec2 = Some(egui::vec2(320.0, 256.0));
    let columns = 8;
    let padding = 0.2;

    let options = eframe::NativeOptions {
        initial_window_size: size_some_vec2,
        min_window_size: size_some_vec2,
        icon_data: None,
        follow_system_theme: true,
        vsync: true,
        ..Default::default()
    };

    let mut active_panel = PanelSelector::Unselected;

    eframe::run_simple_native("KolorWheel.rs", options, move |ctx, _frame| {

        egui::CentralPanel::default().show(ctx, |ui| {
            let mut app_window = AppWindow::new(ui, columns, padding, active_panel.clone());
            app_window.show_panel();
            active_panel = app_window.get_active_panel();
        });
    })

}

struct AppWindow<'u> {
    ui: &'u mut egui::Ui,
    width: f32,
    height: f32,
    columns: u32,
    padding: f32,
    active_panel: PanelSelector,
}

#[derive(Clone)]
enum PanelSelector {
    Unselected, Panel1, Panel2,
}

impl AppWindow<'_> {

    pub fn new(ui: &mut egui::Ui, columns: u32, padding: f32, active_panel: PanelSelector) -> AppWindow {

        let width = ui.available_width();
        let height = ui.available_height();

        AppWindow { ui, width, height, columns, padding, active_panel }
    }


    fn get_active_panel(&self) -> PanelSelector {
        self.active_panel.clone()
    }

    fn show_panel(&mut self) {
        
        if self.ui.selectable_label(
            if let PanelSelector::Panel1 = self.active_panel {true} else {false},
            "P1",
        ).clicked() {
            self.active_panel = PanelSelector::Panel1;
        }

        if self.ui.selectable_label(
            if let PanelSelector::Panel2 = self.active_panel {true} else {false},
            "P2"
        ).clicked() {
            self.active_panel = PanelSelector::Panel2;
        }

        if let PanelSelector::Unselected = self.active_panel {} else {
            self.ui.label(format!(
                "{}x{} - {}", 
                self.width, 
                self.height,
                match self.active_panel {
                    PanelSelector::Unselected => "unselected",
                    PanelSelector::Panel1 => "p1",
                    PanelSelector::Panel2 => "p2",
                }
            ));
        }

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
