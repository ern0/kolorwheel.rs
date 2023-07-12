#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused)]

use eframe::egui;
extern crate kolorwheel;

fn main() -> Result<(), eframe::Error> {

    let size_some_vec2 = Some(egui::vec2(320.0, 256.0));
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
            let mut app = App::new(ui, active_panel.clone(), padding);
            app.show_panel();
            active_panel = app.get_active_panel();
        });
    })

}

struct App<'u> {
    ui: &'u mut egui::Ui,
    active_panel: PanelSelector,
    
    window: Window,
    cell_opt: Option<Cell>,
}

#[derive(Clone, PartialEq)]
enum PanelSelector {
    Panel1, Panel2,
}

impl App<'_> {

    pub fn new(ui: &mut egui::Ui, active_panel: PanelSelector, padding: f32) -> App {

        let width = ui.available_width();
        let height = ui.available_height();
        let rounding = width / 100.0;        
        let window = Window::new(
            width, 
            height, 
            padding, 
            rounding
        );

        App { 
            ui, 
            active_panel, 
            window,
            cell_opt: None,
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

    }

    fn show_panel1(&mut self) {

        self.ui.label("panel 1");
        self.cell_opt = Some(Cell::new(&self.window, 10));

        self.ui.label(format!("{}", self.ui.available_height()));

        let rect = egui::Rect {
            min: egui::Pos2{ x: 50.0, y: 50.0 },
            max: egui::Pos2{ x: 100.0, y: 100.0 },
        };

        let fill = egui::Color32::BLUE;

    }

    fn show_panel2(&mut self) {

        self.ui.label("panel 2");
        self.cell_opt = Some(Cell::new(&self.window, 10));
    }

    fn show_box(&mut self, rect: egui::Rect, fill: egui::Color32) {

        let stroke = egui::epaint::Stroke{
            width: 1.0,
            color: fill,
        };

        let rect_shape = egui::epaint::RectShape { 
            rect, 
            rounding: self.window.rounding, 
            fill, 
            stroke 
        };

        let (_, painter) = self.ui.allocate_painter(
            egui::Vec2::new(
                self.window.width as f32, 
                self.window.height as f32,
            ),
            egui::Sense::hover(),
        );

        let rectangle = egui::Shape::Rect(rect_shape);
        painter.add(rectangle);

    }
}

struct Window {
    width: u32,
    height: u32,
    padding: u32,
    rounding: egui::Rounding,
}

impl Window {

    pub fn new(width: f32, height: f32, padding: f32, rounding: f32) -> Window {

        let egui_rounding = egui::Rounding {
            nw: rounding, 
            ne: rounding,
            sw: rounding, 
            se: rounding,
        };

        Window { 
            width: width as u32, 
            height: height as u32, 
            padding: padding as u32, 
            rounding: egui_rounding 
        }

    }  
}
struct Cell {
    width: u32,
    height: u32,
    padding_top: u32,
    padding_bottom: u32,
    padding_left: u32,
    padding_right: u32,
    box_width: u32,
    box_height: u32,
}

impl Cell {

    pub fn new(window: &Window, columns: u32) -> Cell {

        Cell {
            width: 0,
            height: 0,
            padding_top: 0,
            padding_bottom: 0,
            padding_left: 0,
            padding_right: 0,
            box_width: 0,
            box_height: 0,
        }
    }
}
