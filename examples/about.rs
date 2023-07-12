#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused)]

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

    let mut active_panel = PanelSelector::Panel1;

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
    rounding: egui::Rounding,
}

#[derive(Clone, PartialEq)]
enum PanelSelector {
    Panel1, Panel2,
}

impl AppWindow<'_> {

    pub fn new(ui: &mut egui::Ui, columns: u32, padding: f32, active_panel: PanelSelector) -> AppWindow {

        let width = ui.available_width();
        let height = ui.available_height();

        let box_rounding = width / 100.0;        
        let rounding = egui::Rounding {
            nw: box_rounding, 
            ne: box_rounding,
            sw: box_rounding, 
            se: box_rounding,
        };

        AppWindow { 
            ui, 
            width, 
            height, 
            columns, 
            padding, 
            active_panel, 
            rounding,
        }
    }


    fn get_active_panel(&self) -> PanelSelector {
        self.active_panel.clone()
    }

    fn show_panel(&mut self) {
        
        self.ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
            ui.selectable_value(&mut self.active_panel, PanelSelector::Panel1, "Panel1");
            ui.selectable_value(&mut self.active_panel, PanelSelector::Panel2, "Panel2");
        });        

        self.ui.separator();

        match self.active_panel {
            PanelSelector::Panel1 => self.show_panel1(),
            PanelSelector::Panel2 => self.show_panel2(),
        }

        let rect = egui::Rect {
            min: egui::Pos2{ x: 50.0, y: 50.0 },
            max: egui::Pos2{ x: 100.0, y: 100.0 },
        };

        let fill = egui::Color32::BLUE;

    }

    fn show_panel1(&mut self) {
        self.ui.label("panel 1");
        self.ui.label(format!("{}", self.ui.available_height()));
    }

    fn show_panel2(&mut self) {
        self.ui.label("panel 2");
    }

    fn show_box(&mut self, rect: egui::Rect, fill: egui::Color32) {

        let stroke = egui::epaint::Stroke{
            width: 1.0,
            color: fill,
        };

        let rect_shape = egui::epaint::RectShape { 
            rect, 
            rounding: self.rounding, 
            fill, 
            stroke 
        };

        let (_, painter) = self.ui.allocate_painter(
            egui::Vec2::new(self.width, self.height),
            egui::Sense::hover(),
        );

        let rectangle = egui::Shape::Rect(rect_shape);
        painter.add(rectangle);

    }
}
