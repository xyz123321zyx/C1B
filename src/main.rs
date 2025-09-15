#![allow(warnings)]
mod debugger;
mod errors;
mod models;
mod msg;
mod ui;
use eframe::{egui, NativeOptions};
use image::GenericImageView;

use crate::{
    models::{C1BState, Tab, state},
    ui::C1BUI,
};

fn main() {
    let state = C1BState::new();
    debugger::StateDebugger::state(&state);
    let image = image::open("src/assets/icons/app/Cachatto.ico").expect("Failed to open icon");
    let (width, height) = image.dimensions();
    let rgba = image.into_rgba8().into_raw();

    let icon = egui::IconData {
        rgba,
        width,
        height,
    };

    let native_options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_title("CSB-v1")
            .with_icon(icon),
        ..Default::default()
    };
    eframe::run_native(
        "C1B-v1",
        native_options,
        Box::new(|cc| Ok(Box::new(C1BUI::new(state)))),
    );
}
