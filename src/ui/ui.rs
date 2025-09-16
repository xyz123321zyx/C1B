use std::sync::mpsc::{Receiver, Sender};

use eframe::egui;

use crate::{
    event_handler,
    events::Events,
    models::{C1BState, state},
    ui::{NavBarUI, TabUI},
};

pub struct C1BUI {
    pub state: C1BState,
    pub event_sender: Sender<Events>,
    pub event_receiver: Receiver<Events>,
}

impl C1BUI {
    pub fn new(state: state::C1BState) -> Self {
        let (tx, rx) = std::sync::mpsc::channel();
        Self {
            state,
            event_sender: tx,
            event_receiver: rx,
        }
    }
}

impl eframe::App for C1BUI {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        while let Ok(event) = self.event_receiver.try_recv() {
            event_handler::EventHandler::handle_event(event, &mut self.state);
        }
        egui::TopBottomPanel::top("tab_manager")
            .frame(egui::Frame::none())
            .show(ctx, |ui| {
                ui.set_height(40.0);
                TabUI::render_tab_bar(ui, &self.state, &self.event_sender);
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
            .show(ctx, |ui| {
                
            });
    }
}
