use eframe::egui;

use crate::{
    models::{C1BState, state},
    ui::{NavBarUI, TabUI},
};

pub struct C1BUI {
    pub state: C1BState,
}

impl C1BUI {
    pub fn new(state: state::C1BState) -> Self {
        Self { state }
    }
}

impl eframe::App for C1BUI {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("tab_manager")
            .frame(egui::Frame::none())
            .show(ctx, |ui| {
                ui.set_height(40.0);

                // Tab bar section
                // self.render_tab_bar(ui);
                TabUI::render_tab_bar(ui, &self.state);

                // Navigation controls section
                NavBarUI::render_navigation_bar(ui, &self.state);
            });

        egui::CentralPanel::default()
            .frame(egui::Frame {
                fill: egui::Color32::WHITE,
                stroke: egui::Stroke::NONE,
                shadow: egui::epaint::Shadow::NONE,
                inner_margin: 0.0.into(),
                ..Default::default()
            })
            .show(ctx, |ui| ui.heading("Webview"));
    }
}
