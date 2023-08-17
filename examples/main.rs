#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused)]

use std::boxed::Box;

extern crate kolorwheel;
use kolorwheel::KolorWheel;
use kolorwheel::SpinMode;
use kolorwheel::FadeMode;
use kolorwheel::hsl_color::HslColor;
use kolorwheel::rgb_color::RgbColor;

mod panel1_gradient;
mod panel2_hue_abs;
mod panel3_sat_abs;
mod panel4_lit_abs;
mod panel5_p6_hue_rel_univ;
mod panel7_sat_lit_rel;
mod panel8_hue_offsets;
mod panel9_palette1;
mod panel10_palette2;

fn main() -> Result<(), eframe::Error> {

    const WINDOW_WIDTH: f32 = 720.0;
    const MIN_WIDTH: f32 = 320.0;
    const WINDOW_HEIGHT: f32 = 512.0;
    const MIN_HEIGHT: f32 = 256.0;
    const CELL_PADDING: u32 = 30;

    let mut app = App::new(WINDOW_WIDTH, WINDOW_HEIGHT, CELL_PADDING);

    let eframe_options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(WINDOW_WIDTH, WINDOW_HEIGHT)),
        min_window_size: Some(egui::vec2(MIN_WIDTH, MIN_HEIGHT)),
        icon_data: None,
        follow_system_theme: true,
        vsync: true,
        initial_window_pos: Some(egui::pos2(1500.0, 80.0)), //TODO: remove this line
        ..Default::default()
    };
    eframe::run_simple_native("KolorWheel.rs", eframe_options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            app.show_panel(ui);
        });
    })

}

trait Panel {
    fn paint(&mut self, ui: &mut egui::Ui) -> (KolorWheel, u32, u32);
}
#[derive(Copy, Clone, PartialEq)]
enum PanelSelector {
    Gradient, 
    HueAbs, SatAbs, LitAbs,
    HueReli, HueRelx,
    SatLitRel, HueOffsets,
    Palette1, Palette2,
}

struct App {
    window: Window,
    active_panel: PanelSelector,   
    p1: panel1_gradient::Gradient,
    p2: panel2_hue_abs::HueAbs,
    p3: panel3_sat_abs::SatAbs,
    p4: panel4_lit_abs::LitAbs,
    p5: panel5_p6_hue_rel_univ::HueRelUniv,
    p6: panel5_p6_hue_rel_univ::HueRelUniv,
    p7: panel7_sat_lit_rel::SatLitRel,
    p8: panel8_hue_offsets::HueOffsets,
    p9: panel9_palette1::Palette1,
    p10: panel10_palette2::Palette2,
}

impl App {

    pub fn new(window_width: f32, window_height: f32, cell_padding: u32) -> Self {

        let window = Window::new(
            window_width, 
            window_height, 
            cell_padding, 
            window_width / 100.0
        );

        Self { 
            window,
            active_panel: PanelSelector::Gradient,
            p1: panel1_gradient::Gradient::new(), 
            p2: panel2_hue_abs::HueAbs::new(), 
            p3: panel3_sat_abs::SatAbs::new(), 
            p4: panel4_lit_abs::LitAbs::new(),
            p5: panel5_p6_hue_rel_univ::HueRelUniv::new(true),
            p6: panel5_p6_hue_rel_univ::HueRelUniv::new(false),
            p7: panel7_sat_lit_rel::SatLitRel::new(),
            p8: panel8_hue_offsets::HueOffsets::new(),
            p9: panel9_palette1::Palette1::new(),
            p10: panel10_palette2::Palette2::new(),
        }
    }

    fn show_panel(&mut self, ui: &mut egui::Ui) {

        self.window.original_height = ui.available_height() as u32;

        ui.with_layout(egui::Layout::right_to_left(egui::Align::LEFT), |ui| {
            ui.label(" ");
            ui.hyperlink("https://github.com/ern0/kolorwheel.rs");
        });

        ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
            ui.selectable_value(&mut self.active_panel, PanelSelector::Gradient, "Gradient");
            ui.selectable_value(&mut self.active_panel, PanelSelector::HueAbs, "Hue/abs");
            ui.selectable_value(&mut self.active_panel, PanelSelector::SatAbs, "Sat/abs");
            ui.selectable_value(&mut self.active_panel, PanelSelector::LitAbs, "Lit/abs");
            ui.selectable_value(&mut self.active_panel, PanelSelector::HueReli, "Hue/reli");
            ui.selectable_value(&mut self.active_panel, PanelSelector::HueRelx, "Hue/relx");
            ui.selectable_value(&mut self.active_panel, PanelSelector::SatLitRel, "Sat+Lit/reli");
            ui.selectable_value(&mut self.active_panel, PanelSelector::HueOffsets, "HueOffsets");
            ui.selectable_value(&mut self.active_panel, PanelSelector::Palette1, "Palette1");
            ui.selectable_value(&mut self.active_panel, PanelSelector::Palette2, "Palette2");
        });

        ui.separator();

         let panel: &mut dyn Panel = match self.active_panel {
            PanelSelector::Gradient => &mut self.p1,
            PanelSelector::HueAbs => &mut self.p2,
            PanelSelector::SatAbs => &mut self.p3,
            PanelSelector::LitAbs => &mut self.p4,
            PanelSelector::HueReli => &mut self.p5,
            PanelSelector::HueRelx => &mut self.p6,
            PanelSelector::SatLitRel => &mut self.p7,
            PanelSelector::HueOffsets => &mut self.p8,
            PanelSelector::Palette1 => &mut self.p9,
            PanelSelector::Palette2 => &mut self.p10,
        };

        let (mut kw, cols, rows) = panel.paint(ui);

        // Appended `self.paint_grid()` to this method because borrow issues:
        // "cannot borrow `*self` as mutable more than once at a time"
        // Also made `paint_box()` static method
        //
        // this method's last line:
        //     self.paint_grid(ui, kw, cols, rows);
        // header for fn:
        //     fn paint_grid(&mut self, ui: &mut egui::Ui, kw: KolorWheel, cols: u32, rows: u32) {

        self.window.actual_width = ui.available_width() as u32;
        self.window.actual_height = ui.available_height() as u32;

        let (_, painter) = ui.allocate_painter(
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

            Self::paint_box(&painter, rect, fill, self.window.rounding);

            col += 1;
            x += cell.cell_width;
            if col == cell.columns {
                col = 0;
                y += cell.cell_height;
                x = self.window.left + cell.window_centering_horizontal;
            }
        }  
    }

    fn paint_hsl_sliders(ui: &mut egui::Ui, color: &mut HslColor) {

        let mut h: i32 = color.h as i32;
        let mut s: i32 = color.s as i32;
        let mut l: i32 = color.l as i32;

        let slider_hue = egui::widgets::Slider::new(&mut h, 0..=359)
            .orientation(egui::SliderOrientation::Vertical)
            .trailing_fill(true)
            .text("Hue")
            .suffix("°")
        ;
        ui.add(slider_hue);

        let slider_sat = egui::widgets::Slider::new(&mut s, 0..=100)
            .orientation(egui::SliderOrientation::Vertical)
            .trailing_fill(true)
            .text("Sat")
            .suffix("%")
        ;
        ui.add(slider_sat);

        let slider_lit = egui::widgets::Slider::new(&mut l, 0..=100)
            .orientation(egui::SliderOrientation::Vertical)
            .trailing_fill(true)
            .text("Lit")
            .suffix("%")
        ;
        ui.add(slider_lit);

        color.h = h as f32;
        color.s = s as f32;
        color.l = l as f32;

    }

    fn paint_box(painter: &egui::Painter, rect: egui::Rect, hsl_color: HslColor, rounding: egui::Rounding) {
        
        let rgb_color: RgbColor = hsl_color.into();
        let fill = egui::Color32::from_rgb(rgb_color.r, rgb_color.g, rgb_color.b);

        let stroke = egui::epaint::Stroke{
            width: 1.0,
            color: fill,
        };

        let rect_shape = egui::epaint::RectShape { 
            rect, 
            rounding, 
            fill, 
            stroke,
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
