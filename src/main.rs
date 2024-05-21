#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

mod parse;

use eframe::{egui, egui::IconData};

use egui_plot;

use log::info;
use serialport::available_ports;

use crate::parse::{Data, Parser};
use std::env;

fn main() -> Result<(), eframe::Error> {
    env::set_var("RUST_LOG", "debug");
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([640.0, 320.0])
            .with_icon(load_icon()),
        ..Default::default()
    };
    eframe::run_native(
        "Meister App",
        options,
        Box::new(|_| Box::<MeisterApp>::default()),
    )
}

struct MeisterApp {
    port: String,
    parser: Parser,
}

impl Default for MeisterApp {
    fn default() -> Self {
        Self {
            port: "".to_owned(),
            parser: Parser::new(),
        }
    }
}

impl eframe::App for MeisterApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.parser.parse();
        egui::CentralPanel::default().show(ctx, |ui| {
            ctx.request_repaint_after(std::time::Duration::from_millis(25));

            match self.parser.get_port() {
                Some(_) => {
                    ui.label(format!("Connected to port: {}", self.port));
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        let filename_label = ui.label("filename:");
                        ui.text_edit_singleline(&mut self.parser.filename)
                            .labelled_by(filename_label.id);
                    });

                    for i in 0..3 {
                        if let Some(imu_data) = self.parser.get_imu(i as u8).last() {
                            egui::Window::new(format!("IMU{:02x}", i))
                                .vscroll(true)
                                .show(ctx, |ui| {
                                    imu_data.draw(ui);
                                });
                        }
                    }

                    if let Some(alt_data) = self.parser.get_alt_data().last() {
                        egui::Window::new("Altitude").vscroll(true).show(ctx, |ui| {
                            ui.heading(format!(
                                "altitude:\t{:2.2}m\ttimestamp:\t{}ms",
                                alt_data.altitude / 100.0,
                                alt_data.timestamp
                            ));
                            let plt = egui_plot::Plot::new("Altitude");
                            let point: egui_plot::PlotPoints = self
                                .parser
                                .get_alt_data()
                                .iter()
                                .enumerate()
                                .map(|(n,data)| [n as f64, data.altitude as f64 / 100.0])
                                .collect();
                            let line=egui_plot::Line::new(point)
                                .color(egui::Color32::from_rgb(255, 0, 0))
                                .name("altitude");
                            plt.show(ui, |plot_ui| {
                                plot_ui.line(line);
                            });
                            
                        });
                    }

                    if let Some(servo_data) = self.parser.get_servo_data().last() {
                        egui::Window::new("Servo").vscroll(true).show(ctx, |ui| {
                            servo_data.draw(ui);
                        });
                    }
                }
                None => {
                    ui.horizontal(|ui| match available_ports() {
                        Ok(ports) => {
                            ui.label("Available ports:\t");
                            for port in ports {
                                ui.selectable_value(
                                    &mut self.port,
                                    port.port_name.clone(),
                                    port.port_name.clone(),
                                );
                            }
                        }
                        Err(e) => {
                            println!("Error: {:?}", e);
                        }
                    });

                    if ui
                        .button("Connect")
                        .on_hover_text("Connect to the selected port")
                        .clicked()
                    {
                        match serialport::new(&self.port, 115200).open() {
                            Ok(port) => {
                                info!("Connected to port: {}", self.port);
                                self.parser.set_port(port);
                            }
                            Err(e) => {
                                println!("Error: {:?}", e);
                                ui.label("Failed to connect");
                            }
                        }
                    };
                }
            }
        });
    }
}

pub(crate) fn load_icon() -> IconData {
    let (icon_rgba, icon_width, icon_height) = {
        let icon = include_bytes!("../assets/logo.png");
        let image = image::load_from_memory(icon)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    }
}
