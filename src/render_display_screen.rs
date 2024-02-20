use std::sync::Arc;
use eframe::egui::{Align, Button, Color32, Label, Layout, RichText, ScrollArea, Ui};
use crate::Model::{DisplayPosition, MainBody, remove_element_sql};

impl MainBody {
    pub fn render_display_screen(&mut self, ui : &mut Ui, full_width : f32, full_height : f32) -> () {
        match self.display_storage.storage_vector.len() {
            0 => {
                ui.vertical(|ui| {
                    ui.add_space(full_height * 0.4f32);
                    ui.horizontal(|ui| {
                        ui.add_space(full_width * 0.25f32);
                        ui.add(Label::new(RichText::new("There are no snippets. Add something before proceeding").heading().color(Color32::WHITE).monospace()))
                    });
                    ui.add_space(full_height * 0.1f32);
                    ui.horizontal(|ui| {
                        ui.add_space(full_width * 0.18f32);
                        ui.add(Label::new(RichText::new(format!("Display would be updated in {} secs", self.update_timer.countdown)).color(Color32::WHITE).size(25f32).monospace()))
                    });
                });
                // if self.update_timer.countdown == 7 && !self.update_timer.active {
                //     spawn_update_timer(self.update_timer.countdown_sender.clone(), self.update_timer.sender_active.clone())
                // }
            },
            _ => {
                ui.vertical(|ui| {
                    ui.add_space(full_height * 0.05f32);
                    ScrollArea::vertical().max_width(full_width).max_height(full_height).show(ui, |ui| {
                        for (count, element) in self.display_storage.storage_vector.iter().enumerate() {
                            if count == 0 {
                                ui.separator();
                            }
                            ui.horizontal(|ui| {
                                ui.add_space(full_width * 0.1f32);
                                ui.add(Label::new(RichText::new(format!("{}.", element.sample_name)).color(Color32::WHITE).underline()));
                                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                    ui.add_space(full_width * 0.1f32);
                                    ui.add(Label::new(RichText::new(format!("{}", element.language_type)).color(Color32::WHITE).underline()));
                                })
                            });
                            ui.add_space(full_height * 0.05f32);
                            ui.horizontal(|ui| {
                                ui.add_space(full_width * 0.1f32);
                                if ui.add_sized([full_width * 0.1f32, full_height * 0.04f32], Button::new(RichText::new("Delete"))).clicked() {
                                    remove_element_sql(Arc::clone(&self.sql_connection), self.logs_body.log_sender.clone(), element.id, self.display_storage.storage_sender.clone())
                                }
                                ui.add_space(full_width * 0.05f32);
                                if ui.add_sized([full_width * 0.1f32, full_height * 0.04f32], Button::new(RichText::new("Examine"))).clicked() {
                                    self.user_input.libraries = element.library_sample.clone();
                                    self.user_input.sample_name = element.sample_name.clone();
                                    self.user_input.code_to_save = element.sample.clone();
                                    self.user_input.language_name = element.language_type.clone();
                                    self.display_position = DisplayPosition::Concrete;
                                }
                            });
                            ui.separator();
                        }
                    });
                });
            }
        }
    }
}