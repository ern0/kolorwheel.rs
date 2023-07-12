#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;

extern crate kolorwheel;

fn main() -> Result<(), eframe::Error> {

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };

    let mut name = "Arthur".to_owned();
    let mut age = 42;

    eframe::run_simple_native("KolorWheel.rs", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            panel(ui, &mut name, &mut age);
        });
    })
}

fn panel(ui: &mut egui::Ui, name: &mut String, age: &mut i32) {

    ui.heading("KolorWheel.rs");
    ui.horizontal(|ui| {
        let name_label = ui.label("Your name: ");
        ui.text_edit_singleline(name).labelled_by(name_label.id);
    });
    ui.add(egui::Slider::new(age, 0..=120).text("age"));
    if ui.button("Click each year").clicked() {
        *age += 1;
    }
    ui.label(format!("Hello '{name}', age {age}"));

}
