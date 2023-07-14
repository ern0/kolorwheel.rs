#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused)]

extern crate kolorwheel;

use eframe::egui;
use kolorwheel::KolorWheel;

fn main() -> Result<(), eframe::Error> {

    let padding_percent = 30;

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(480.0, 320.0)),
        min_window_size: Some(egui::vec2(320.0, 256.0)),
        icon_data: None,
        follow_system_theme: true,
        vsync: true,
        initial_window_pos: Some(egui::pos2(600.0, 80.0)), //TODO: remove this line
        ..Default::default()
    };

    let mut active_panel = PanelSelector::Panel1;

    eframe::run_simple_native("KolorWheel.rs", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut app = App::new(ui, active_panel.clone(), padding_percent);
            app.show_panel();
            active_panel = app.get_active_panel();
        });
    })

}

struct App<'u> {
    ui: &'u mut egui::Ui,
    active_panel: PanelSelector,   
    window: Window,
}

#[derive(Clone, PartialEq)]
enum PanelSelector {
    Panel1, Panel2,
}

impl App<'_> {

    pub fn new(ui: &mut egui::Ui, active_panel: PanelSelector, padding_percent: u32) -> App {

        let width = ui.available_width();
        let height = ui.available_height();
        let rounding = width / 100.0;        
        let window = Window::new(
            width, 
            height, 
            padding_percent, 
            rounding,
        );

        App { 
            ui, 
            active_panel, 
            window,
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
            PanelSelector::Panel1 => self.paint_panel1(10, 10),
            PanelSelector::Panel2 => self.paint_panel2(4, 4),
        }

    }

    fn paint_panel1(&mut self, cols: u32, rows: u32) {

        let kw = KolorWheel::new()
            .set_count(cols * rows)
            .set_rgb(255, 0, 0)
            .hue_values(&[70, 80, 90])
            .lit_abs(50) //TODO wtf
        ;

        self.paint_grid(kw, cols, rows);
    }

    fn paint_panel2(&mut self, cols: u32, rows: u32) {

        self.ui.label("panel 2");
        self.ui.label("blah blah blah\nblah blah");
        
        let kw = KolorWheel::new()
            .set_count(cols * rows)
            .set_hsl(180.0, 40.0, 90.0) 
        ;
        self.paint_grid(kw, cols, rows);
    }

    fn paint_grid(&mut self, kw: KolorWheel, cols: u32, rows: u32) {

        self.window.update_dims(
            self.ui.available_width() as u32, 
            self.ui.available_height() as u32,
        );

        let (_, painter) = self.ui.allocate_painter(
            egui::Vec2::new(
                self.window.actual_width as f32, 
                self.window.actual_height as f32,
            ),
            egui::Sense::hover(),
        );

        let cell = Cell::new(&self.window, cols, rows);

        let mut col = 0;
        let mut x = self.window.left + cell.window_centering_horizontal;
        let header_height = self.window.original_height - self.window.actual_height;
        let mut y = self.window.top + header_height;


        for fill in kw {

            let rect = egui::Rect {
                min: egui::Pos2{
                    x: (x + cell.cell_padding) as f32,
                    y: (y + cell.cell_padding) as f32,
                },
                max: egui::Pos2 { 
                    x: (x + cell.cell_padding + cell.cell_padded_width) as f32,
                    y: (y + cell.cell_padding + cell.cell_padded_height) as f32,
                },
            };

            self.paint_box(&painter, rect, fill);

            col += 1;
            x += cell.cell_width;
            if col == cell.columns {
                col = 0;
                y += cell.cell_height;
                x = self.window.left + cell.window_centering_horizontal;
            }
        }        
    }

    fn paint_box(&self, painter: &egui::Painter, rect: egui::Rect, fill: egui::Color32) {

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

        let rectangle = egui::Shape::Rect(rect_shape);
        painter.add(rectangle);

    }
}

struct Window {
    original_height: u32,
    actual_width: u32,
    actual_height: u32,
    cell_padding_percent: u32,
    rounding: egui::Rounding,
    left: u32,
    top: u32,
}

impl Window {

    pub fn new(width: f32, height: f32, cell_padding_percent: u32, rounding: f32) -> Window {

        let egui_rounding = egui::Rounding {
            nw: rounding, 
            ne: rounding,
            sw: rounding, 
            se: rounding,
        };

        Window { 
            original_height: height as u32, 
            actual_width: width as u32, 
            actual_height: height as u32, 
            cell_padding_percent, 
            rounding: egui_rounding,
            left: 6,  // magic value
            top: 9,   // magic value
        }
    }  

    pub fn update_dims(&mut self, width: u32, height: u32) {
        self.actual_width = width;
        self.actual_height = height;
    }

}
struct Cell {
    columns: u32,
    _rows: u32,
    _window_corrected_width: u32,
    window_centering_horizontal: u32,
    _window_corrected_height: u32,  // used by tests
    cell_width: u32,
    cell_height: u32,
    cell_padding: u32,
    cell_padded_width: u32,
    cell_padded_height: u32,
}

impl Cell {

    pub fn new(window: &Window, columns: u32, rows: u32) -> Cell {

        let cell_width = window.actual_width / columns;
        let window_corrected_width = cell_width * columns;
        let window_padding_horizontal = (window.actual_width - window_corrected_width) / 2;
        let mut padding_horizontal = (cell_width * window.cell_padding_percent) / 200;
        if padding_horizontal < 2 { 
            padding_horizontal = 2;
        }

        let cell_height = window.actual_height / rows;
        let window_corrected_height = cell_height * rows;
        let mut padding_vertical = (cell_height * window.cell_padding_percent) / 200;
        if padding_vertical < 2 {
            padding_vertical = 2;
        }

        let cell_padding = {
            if padding_horizontal > padding_vertical {
                padding_vertical
            } else {
                padding_horizontal
            }
        };
        let cell_padded_width = cell_width - (cell_padding * 2);
        let cell_padded_height = cell_height - (cell_padding * 2);

        Cell {
            columns,
            _rows: rows,
            _window_corrected_width: window_corrected_width, 
            window_centering_horizontal: window_padding_horizontal,
            _window_corrected_height: window_corrected_height,
            cell_width,
            cell_height,
            cell_padding,
            cell_padded_width,
            cell_padded_height,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]    
    fn test_window_corrected_width_exact() {

        let window = Window::new(320.0, 200.0, 0, 0.0);
        let cell = Cell::new(&window, 10, 10);

        assert_eq!(cell.cell_width, 32);
        assert_eq!(cell._window_corrected_width, 320);
        assert_eq!(cell.window_centering_horizontal, 0);
    }

    #[test]
    fn test_window_corrected_width_round() {

        let window = Window::new(324.0, 200.0, 0, 0.0);
        let cell = Cell::new(&window, 10, 10);

        assert_eq!(cell.cell_width, 32);
        assert_eq!(cell._window_corrected_width, 320);
        assert_eq!(cell.window_centering_horizontal, 2);
    }

    #[test]
    fn test_window_horizontal_padding() {

        let window = Window::new(320.0, 200.0, 0, 0.0);
        let cell = Cell::new(&window, 10, 10);

        let sum = cell.cell_padded_width + (cell.cell_padding * 2);
        assert_eq!(cell.cell_width, sum);
    }

    #[test]    
    fn test_window_corrected_height_exact() {

        let window = Window::new(320.0, 200.0, 0, 0.0);
        let cell = Cell::new(&window, 10, 10);

        assert_eq!(cell.cell_height, 20);
        assert_eq!(cell._window_corrected_height, 200);
    }

    #[test]
    fn test_window_corrected_height_round() {

        let window = Window::new(320.0, 202.0, 0, 0.0);
        let cell = Cell::new(&window, 10, 10);

        assert_eq!(cell.cell_height, 20);
        assert_eq!(cell._window_corrected_height, 200);
    }

    #[test]
    fn test_window_vertical_padding() {

        let window = Window::new(320.0, 200.0, 0, 0.0);
        let cell = Cell::new(&window, 10, 10);

        let sum = cell.cell_padded_height + (cell.cell_padding * 2);
        assert_eq!(cell.cell_height, sum);
    }

}
