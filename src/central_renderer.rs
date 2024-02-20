use std::sync::Arc;
use eframe::egui::{Button, Color32, Label, RichText, ScrollArea, Ui};
use crate::Model::{DisplayPosition, get_data, MainBody, send_data};

impl MainBody {
    pub fn render_central(&mut self, ui : &mut Ui, full_width : f32, full_height : f32) -> () {
        ui.vertical(|ui| {
            ui.add_space(full_height * 0.05f32);
            ui.horizontal(|ui| {
                ui.add_space(full_width * 0.05f32);
                ui.vertical(|ui| {
                    ui.add(Label::new(RichText::new("Enter your language:").monospace().color(Color32::WHITE)));
                    ui.add_space(full_height * 0.008f32);
                    ui.add_sized([full_width * 0.18f32, full_height * 0.04f32], eframe::egui::TextEdit::singleline(&mut self.user_input.language_name))
                });
                ui.add_space(ui.available_width() * 0.05f32);
                ui.vertical(|ui| {
                    ui.add(Label::new(RichText::new("Enter name for your sample:").monospace().color(Color32::WHITE)));
                    ui.add_space(full_height * 0.008f32);
                    ui.add_sized([full_width * 0.25f32, full_height * 0.04f32], eframe::egui::TextEdit::singleline(&mut self.user_input.sample_name))
                });
                ui.scope(|ui| {
                    let mut style = ui.style_mut();
                    style.visuals.widgets.hovered.weak_bg_fill = Color32::RED;
                    style.visuals.widgets.inactive.weak_bg_fill = Color32::BLACK;
                    ui.add_space(ui.available_width() * 0.1f32);
                    ui.vertical(|ui| {
                        ui.add_space(full_height * 0.036f32);
                        if ui.add_sized([full_width * 0.1f32, full_height * 0.04f32], Button::new(RichText::new("Clear").color(Color32::WHITE).italics().underline())).clicked() {
                            self.clear_fields();
                        }
                    });
                });
                ui.scope(|ui| {
                    let mut style = ui.style_mut();
                    style.visuals.widgets.hovered.weak_bg_fill = Color32::GREEN;
                    style.visuals.widgets.inactive.weak_bg_fill = Color32::BLACK;
                    ui.add_space(ui.available_width() * 0.1f32);
                    ui.vertical(|ui| {
                        ui.add_space(full_height * 0.036f32);
                        if ui.add_sized([full_width * 0.1f32, full_height * 0.04f32], Button::new(RichText::new("Send").color(Color32::WHITE).italics().underline())).clicked() {
                            send_data(Arc::clone(&self.sql_connection), self.logs_body.log_sender.clone(), self.user_input.language_name.clone(), self.user_input.sample_name.clone(), self.user_input.code_to_save
                                .clone(), self.user_input.libraries.clone());
                            self.clear_fields();
                        }
                    });
                });
            });
            ui.add_space(full_height * 0.05f32);
            ScrollArea::vertical().max_width(full_width).max_height(full_height * 0.8f32).show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.add_space(full_width * 0.05f32);
                    ui.vertical(|ui| {
                        ui.add(Label::new(RichText::new("Paste your code sample here:").monospace().underline().color(Color32::WHITE)));
                        ui.add_space(full_height * 0.008f32);
                        ui.add_sized([full_width * 0.9f32, full_height * 0.5f32], eframe::egui::TextEdit::multiline(&mut self.user_input.code_to_save))
                    });
                });
                ui.add_space(full_height * 0.05f32);
                ui.horizontal(|ui| {
                    ui.add_space(full_width * 0.05f32);
                    ui.vertical(|ui| {
                        ui.add(Label::new(RichText::new("Paste used libraries sample here:").monospace().underline().color(Color32::WHITE)));
                        ui.add_space(full_height * 0.008f32);
                        ui.add_sized([full_width * 0.9f32, full_height * 0.5f32], eframe::egui::TextEdit::multiline(&mut self.user_input.libraries))
                    });
                });
                ui.add_space(full_height * 0.025f32);
            });
        });
    }
}


impl MainBody {
    pub fn render_concrete(&mut self, ui : &mut Ui, full_width : f32, full_height : f32) -> () {
        ui.vertical(|ui| {
            ui.add_space(full_height * 0.05f32);
            ui.horizontal(|ui| {
                ui.add_space(full_width * 0.05f32);
                ui.vertical(|ui| {
                    ui.add(Label::new(RichText::new("Chosen Language").monospace().color(Color32::WHITE)));
                    ui.add_space(full_height * 0.008f32);
                    ui.add_sized([full_width * 0.18f32, full_height * 0.04f32], eframe::egui::TextEdit::singleline(&mut self.user_input.language_name))
                });
                ui.add_space(ui.available_width() * 0.05f32);
                ui.vertical(|ui| {
                    ui.add(Label::new(RichText::new("Sample name:").monospace().color(Color32::WHITE)));
                    ui.add_space(full_height * 0.008f32);
                    ui.add_sized([full_width * 0.25f32, full_height * 0.04f32], eframe::egui::TextEdit::singleline(&mut self.user_input.sample_name))
                });
                ui.scope(|ui| {
                    let mut style = ui.style_mut();
                    style.visuals.widgets.hovered.weak_bg_fill = Color32::GREEN;
                    style.visuals.widgets.inactive.weak_bg_fill = Color32::BLACK;
                    ui.add_space(ui.available_width() * 0.1f32);
                    ui.vertical(|ui| {
                        ui.add_space(full_height * 0.036f32);
                        if ui.add_sized([full_width * 0.1f32, full_height * 0.04f32], Button::new(RichText::new("Return").color(Color32::WHITE).italics().underline())).clicked() {
                            self.display_position = DisplayPosition::Display;
                            get_data(Arc::clone(&self.sql_connection), self.logs_body.log_sender.clone(), self.display_storage.storage_sender.clone());
                            self.clear_fields();
                        }
                    });
                });
            });
            ui.add_space(full_height * 0.05f32);
            ScrollArea::vertical().max_width(full_width).max_height(full_height * 0.8f32).show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.add_space(full_width * 0.05f32);
                    ui.vertical(|ui| {
                        ui.add(Label::new(RichText::new("Your code sample:").monospace().underline().color(Color32::WHITE)));
                        ui.add_space(full_height * 0.008f32);
                        ui.add_sized([full_width * 0.9f32, full_height * 0.5f32], eframe::egui::TextEdit::multiline(&mut self.user_input.code_to_save))
                    });
                });
                ui.add_space(full_height * 0.05f32);
                ui.horizontal(|ui| {
                    ui.add_space(full_width * 0.05f32);
                    ui.vertical(|ui| {
                        ui.add(Label::new(RichText::new("Used libraries sample:").monospace().underline().color(Color32::WHITE)));
                        ui.add_space(full_height * 0.008f32);
                        ui.add_sized([full_width * 0.9f32, full_height * 0.5f32], eframe::egui::TextEdit::multiline(&mut self.user_input.libraries))
                    });
                });
                ui.add_space(full_height * 0.025f32);
            });
        });
    }
}