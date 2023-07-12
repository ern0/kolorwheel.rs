#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused)]

use eframe::egui;
extern crate kolorwheel;

fn main() -> Result<(), eframe::Error> {

    let size_some_vec2 = Some(egui::vec2(320.0, 256.0));
    let padding = 0.2;

    let options = eframe::NativeOptions {
        initial_window_size: size_some_vec2,
        ////min_window_size: size_some_vec2,
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
    opt_cell: Option<Cell>,
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
            opt_cell: None,
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
        self.opt_cell = Some(Cell::new(&self.window, 10, 10));

        self.ui.label(format!("{}", self.ui.available_height()));

        let rect = egui::Rect {
            min: egui::Pos2{ x: 50.0, y: 50.0 },
            max: egui::Pos2{ x: 100.0, y: 100.0 },
        };

        let fill = egui::Color32::BLUE;

    }

    fn show_panel2(&mut self) {

        self.ui.label("panel 2");
        self.opt_cell = Some(Cell::new(&self.window, 10, 10));
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
    window_corrected_width: u32,
    window_centering_horizontal: u32,
    window_corrected_height: u32,
    cell_width: u32,
    cell_height: u32,
    padding_horizontal: u32,
    padding_vertical: u32,
    padded_width: u32,
    padded_height: u32,
}

impl Cell {

    pub fn new(window: &Window, columns: u32, rows: u32) -> Cell {

        let cell_width = window.width / columns;
        let window_corrected_width = cell_width * columns;
        let window_padding_horizontal = (window.width - window_corrected_width) / 2;
        let mut padding_horizontal = (cell_width * 10) / 200;
        if padding_horizontal < 1 { 
            padding_horizontal = 1;
        }
        let padded_width = cell_width - (padding_horizontal * 2);

        let cell_height = window.height / rows;
        let window_corrected_height = cell_height * rows;
        let mut padding_vertical = (cell_height * 10) / 200;
        if padding_vertical < 1 {
            padding_vertical = 1;
        }
        let padded_height = cell_height - (padding_vertical * 2);

        Cell {
            window_corrected_width,
            window_centering_horizontal: window_padding_horizontal,
            window_corrected_height,
            cell_width,
            cell_height,
            padding_horizontal,
            padding_vertical,
            padded_width,
            padded_height,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]    
    fn test_window_corrected_width_exact() {

        let window = Window::new(320.0, 200.0, 0.0, 0.0);
        let cell = Cell::new(&window, 10, 10);

        assert_eq!(cell.cell_width, 32);
        assert_eq!(cell.window_corrected_width, 320);
        assert_eq!(cell.window_centering_horizontal, 0);
    }

    #[test]
    fn test_window_corrected_width_round() {

        let window = Window::new(324.0, 200.0, 0.0, 0.0);
        let cell = Cell::new(&window, 10, 10);

        assert_eq!(cell.cell_width, 32);
        assert_eq!(cell.window_corrected_width, 320);
        assert_eq!(cell.window_centering_horizontal, 2);
    }

    #[test]
    fn test_window_horizontal_padding() {

        let window = Window::new(320.0, 200.0, 0.0, 0.0);
        let cell = Cell::new(&window, 10, 10);

        let sum = cell.padded_width + (cell.padding_horizontal * 2);
        assert_eq!(cell.cell_width, sum);
    }

    #[test]    
    fn test_window_corrected_height_exact() {

        let window = Window::new(320.0, 200.0, 0.0, 0.0);
        let cell = Cell::new(&window, 10, 10);

        assert_eq!(cell.cell_height, 20);
        assert_eq!(cell.window_corrected_height, 200);
    }

    #[test]
    fn test_window_corrected_height_round() {

        let window = Window::new(320.0, 202.0, 0.0, 0.0);
        let cell = Cell::new(&window, 10, 10);

        assert_eq!(cell.cell_height, 20);
        assert_eq!(cell.window_corrected_height, 200);
    }

    #[test]
    fn test_window_vertical_padding() {

        let window = Window::new(320.0, 200.0, 0.0, 0.0);
        let cell = Cell::new(&window, 10, 10);

        let sum = cell.padded_height + (cell.padding_vertical * 2);
        assert_eq!(cell.cell_height, sum);
    }

}
