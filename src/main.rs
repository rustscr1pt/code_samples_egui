#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;
use std::time::Duration;
use eframe::{App, Frame, NativeOptions};
use eframe::egui::{CentralPanel, Color32, Context, TopBottomPanel};
use eframe::egui::panel::TopBottomSide;
use mysql::{Pool, PooledConn};
use tokio::runtime::Runtime;
use tokio::sync::Mutex;
use crate::Model::{DisplayPosition, get_data, MainBody, new};

mod Model;
mod central_renderer;
mod bottom_renderer;
mod render_display_screen;

fn establish_connection() -> PooledConn {
    let pool = Pool::new(url).expect("Couldn't connect to a base");
    println!("Connection with MySQL pool is established!");
    return pool.get_conn().unwrap();
}

impl App for MainBody {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        let base = eframe::egui::containers::Frame {
            inner_margin: Default::default(),
            outer_margin: Default::default(),
            rounding: Default::default(),
            shadow: Default::default(),
            fill: Color32::from_rgb(54, 69,79),
            stroke: Default::default(),
        };

        if let Ok(bool) = self.update_timer.receiver_active.try_recv() {
            self.update_timer.active = bool
        }

        if let Ok(count) = self.update_timer.countdown_receiver.try_recv() {
            if self.update_timer.countdown == 0 {
                get_data(Arc::clone(&self.sql_connection), self.logs_body.log_sender.clone(), self.display_storage.storage_sender.clone(), self.filtered_storage.filtered_sender.clone())
            }
            else {
                self.update_timer.countdown -= count
            }
        }

        if let Ok(filtered) = self.filtered_storage.filtered_receiver.try_recv() {
            self.filtered_storage.filtered_vector = filtered;
        }

        if let Ok(vector) = self.display_storage.storage_receiver.try_recv() {
            self.display_storage.storage_vector = vector
        }

        if let Ok(value) = self.logs_body.log_receiver.try_recv() {
            self.logs_body.storage.push(value.clone());
            self.analyze_clear(value)
        }

        let full_width : f32 = ctx.available_rect().width();
        let full_height : f32 = ctx.available_rect().height();

        match self.display_position {
            DisplayPosition::Display => {
                TopBottomPanel::new(TopBottomSide::Bottom, "logger").exact_height(full_height * 0.2f32).show(ctx, |ui| {
                    self.render_bottom_panel(ui, full_width, full_height)
                });
                CentralPanel::default().frame(base).show(ctx, |ui| {
                    self.render_display_screen(ui, full_width, full_height);
                });
            }
            DisplayPosition::Add => {
                CentralPanel::default().frame(base).show(ctx, |ui| {
                    self.render_central(ui, full_width, full_height)
                });
            }
            DisplayPosition::Concrete => {
                CentralPanel::default().frame(base).show(ctx, |ui| {
                    self.render_concrete(ui, full_width, full_height)
                });
            }
        }
        if self.display_position == DisplayPosition::Add {
            TopBottomPanel::new(TopBottomSide::Bottom, "logger").exact_height(full_height * 0.2f32).show(ctx, |ui| {
                self.render_bottom_panel(ui, full_width, full_height)
            });
        }
    }
}

fn main() {
    let runtime = Runtime::new().unwrap();
    let _enter = runtime.enter();

    std::thread::spawn(move || {
        runtime.block_on(async {
            loop {
                tokio::time::sleep(Duration::from_secs(3600)).await
            }
        })
    });

    let window = NativeOptions::default();
    eframe::run_native("Code Samples", window,
                       Box::new(|cc|
                       Box::new(new(Arc::new(Mutex::new(establish_connection()))))
                       )).unwrap();
}