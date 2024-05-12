#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

mod parse;

use eframe::egui;
use egui_dropdown::DropDownBox;
use log::info;
use serialport::available_ports;

use crate::parse::Parser;
use std::{env, fmt::format};

fn main() -> Result<(), eframe::Error> {
    env::set_var("RUST_LOG", "info");
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
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

                    egui::Window::new("IMU").vscroll(true).show(ctx, |ui| {
                        if let Some(imu) = self.parser.get_imu().last() {
                            ui.label(format!("id: {}", imu.id));
                            ui.label(format!("timestamp: {}", imu.timestamp));
                            ui.label(format!("q_w: {}", imu.q_w));
                            ui.label(format!("q_x: {}", imu.q_x));
                            ui.label(format!("q_y: {}", imu.q_y));
                            ui.label(format!("q_z: {}", imu.q_z));
                        }else{
                            ui.label("No IMU data available");
                        }
                    });

                    egui::Window::new("Altitude").vscroll(true).show(ctx, |ui| {
                        if let Some(alt) = self.parser.get_alt_data().last() {
                            ui.label(format!("id: {}", alt.id));
                            ui.label(format!("timestamp: {}", alt.timestamp));
                            ui.label(format!("altitude: {}", alt.altitude));
                        }else{
                            ui.label("No Altitude data available");
                        }
                    });

                    egui::Window::new("Servo Data")
                        .vscroll(true)
                        .show(ctx, |ui| {
                            if let Some(servo_data) = self.parser.get_servo_data().last() {
                                ui.label(format!("id: {}", servo_data.id));
                                ui.label(format!("timestamp: {}", servo_data.timestamp));
                                ui.label(format!("rudder: {}", servo_data.rudder));
                                ui.label(format!("elevator: {}", servo_data.elevator));
                                ui.label(format!("voltage: {}", servo_data.voltage));
                                ui.label(format!("current_rudder: {}", servo_data.current_rudder));
                                ui.label(format!(
                                    "current_elevator: {}",
                                    servo_data.current_elevator
                                ));
                                ui.label(format!("trim: {}", servo_data.trim));
                                ui.label(format!("status: {}", servo_data.status));
                            }else{
                                ui.label("No Servo data available");
                            }
                        });
                }
                None => {
                    ui.add(
                        DropDownBox::from_iter(
                            match available_ports() {
                                Ok(ports) => ports
                                    .iter()
                                    .into_iter()
                                    .map(|port| port.port_name.clone())
                                    .collect(),
                                Err(e) => {
                                    println!("Error: {:?}", e);
                                    vec![]
                                }
                            },
                            "port",
                            &mut self.port,
                            |ui, text| ui.selectable_label(false, text),
                        )
                        // choose whether to select all text in the text edit when it gets focused
                        // default is false when this is not used
                        .select_on_focus(true)
                        // passes through the desired width to the text edit
                        // default is None internally, so TextEdit does whatever its default implements
                        .desired_width(250.0),
                    );

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
                            }
                        }
                    };
                }
            }
        });
    }
}
