use std::sync::Arc;
use eframe::egui::{Button, Color32, Label, RichText, ScrollArea, Ui};
use crate::Model::{DisplayPosition, get_data, MainBody};

impl MainBody {
    pub fn render_bottom_panel(&mut self, ui : &mut Ui, full_width : f32, full_height : f32) -> () {
        ui.vertical(|ui| {
            ui.add_space(full_height * 0.02f32);
            ui.horizontal(|ui| {
                ui.add_space(full_width * 0.025f32);
                ui.add(Label::new(RichText::new("Logs").color(Color32::WHITE).size(16f32).monospace()));
                ui.add_space(full_width * 0.5f32);
                if ui.add_sized([full_width * 0.1f32, full_height * 0.04f32], Button::new(RichText::new("Display"))).clicked() {
                    get_data(Arc::clone(&self.sql_connection), self.logs_body.log_sender.clone(), self.display_storage.storage_sender.clone(), self.filtered_storage.filtered_sender.clone());
                    self.display_position = DisplayPosition::Display;
                };
                ui.add_space(full_width * 0.035f32);
                if ui.add_sized([full_width * 0.1f32, full_height * 0.04f32], Button::new(RichText::new("Add"))).clicked() {
                    self.display_position = DisplayPosition::Add;
                };
            });
            ScrollArea::vertical().max_width(full_width).max_height(full_height * 0.16f32).show(ui, |ui| {
                if self.logs_body.storage.len() == 0 {
                    ui.horizontal(|ui| {
                        ui.add_space(full_width * 0.025f32);
                        ui.add(Label::new(RichText::new("· There is nothing to show.".to_string()).color(Color32::WHITE).size(12f32).monospace()))
                    });
                }
                else {
                    for elements in &self.logs_body.storage {
                        ui.horizontal(|ui| {
                            ui.add_space(full_width * 0.025f32);
                            ui.add(Label::new(RichText::new(format!("· {}", elements)).color(Color32::WHITE).size(12f32).monospace()))
                        });
                    }
                }
            })
        });
    }
}