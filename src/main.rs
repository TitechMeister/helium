#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

mod parse;
mod ui;

use eframe::egui::{self, FontData, FontDefinitions, FontFamily, IconData};

use log::info;
use serialport::available_ports;
use ui::{
    altitude::AltitudeUI,
    // flight_menu::FlightMenu,
    gps::Gps,
    imu::IMUUI,
    pitot::PitotUI,
    servo::ServoUI,
    tachometer::TachUI,
    vane::VaneUI,
    AppUI
};

use crate::parse::Parser;
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
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::<MeisterApp>::default()
        }),
    )
}

struct MeisterApp {
    port: String,
    parser: Parser,

    alt: AltitudeUI,
    gps: Gps,
    imu: [IMUUI; 4],
    pitot: PitotUI,
    servo: ServoUI,
    tach: TachUI,
    vane: VaneUI,
    // menu: FlightMenu,
}

impl Default for MeisterApp {
    fn default() -> Self {
        Self {
            port: "".to_owned(),
            parser: Parser::new(),
            alt: AltitudeUI::new(),
            gps: Gps::new(),
            imu: [
                IMUUI::new(0x40),
                IMUUI::new(0x41),
                IMUUI::new(0x42),
                IMUUI::new(0x43),
            ],
            pitot: PitotUI::new(),
            servo: ServoUI::new(),
            tach: TachUI::new(),
            vane: VaneUI::new(),

            // menu: FlightMenu::new(),
        }
    }
}

impl eframe::App for MeisterApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "NotoSansJP-Regular".to_owned(),
            FontData::from_static(include_bytes!("../assets/fonts/NotoSansJP-Regular.ttf")),
        );
        fonts
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "NotoSansJP-Regular".to_owned());

        // Put my font as last fallback for monospace:
        fonts
            .families
            .get_mut(&FontFamily::Monospace)
            .unwrap()
            .push("NotoSansJP-Regular".to_owned());
        ctx.set_fonts(fonts);
        self.parser.parse();
        egui::CentralPanel::default().show(ctx, |ui| {
            ctx.request_repaint_after(std::time::Duration::from_millis(25));

            // self.menu.update(&mut self.parser, ctx);

            match self.parser.get_port() {
                Some(_) => {
                    ui.label(format!("Connected to port: {}", self.port));
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        let filename_label = ui.label("filename:");
                        ui.text_edit_singleline(&mut self.parser.filename)
                            .labelled_by(filename_label.id);
                    });

                    self.alt.update(&mut self.parser, ctx);
                    self.gps.update(&mut self.parser, ctx);
                    for imu in &mut self.imu {
                        imu.update(&mut self.parser, ctx);
                    }
                    self.pitot.update(&mut self.parser, ctx);
                    self.servo.update(&mut self.parser, ctx);
                    self.tach.update(&mut self.parser, ctx);
                    self.vane.update(&mut self.parser, ctx);

                    // 生存確認
                    let t = chrono::Utc::now().timestamp_millis();

                    if let Some((_, timestamp)) = self.parser.get_servo_data().last() {
                        if t < (*timestamp + 1000) {
                            ui.colored_label(egui::Color32::GREEN , "0x10:Servo");
                        } else {
                            ui.colored_label(egui::Color32::RED, "0x10:Servo");
                        }
                    }
                    ui.horizontal(|ui| {
                        if let Some((_, timestamp)) = self.parser.get_tach_data(0).last() {
                            if t < *timestamp + 1000 {
                                ui.colored_label(egui::Color32::GREEN , "0x20:Thrust");
                            } else {
                                ui.colored_label(egui::Color32::RED, "0x20:Thrust");
                            }
                        }
                        if let Some((_, timestamp)) = self.parser.get_tach_data(1).last() {
                            if t < *timestamp + 1000 {
                                ui.colored_label(egui::Color32::GREEN , "0x21:Tachometer");
                            } else {
                                ui.colored_label(egui::Color32::RED, "0x21:Tachometer");
                            }
                        }
                    });
                    if let Some((pitot, timestamp)) = self.parser.get_pitot_data().last() {
                        if t < *timestamp + 1000 {
                            ui.colored_label(egui::Color32::GREEN , format!("0x{:02x}:Pitot", pitot.id));
                        } else {
                            ui.colored_label(egui::Color32::RED, format!("0x{:02x}:Pitot", pitot.id));
                        }
                    }

                    ui.horizontal(|ui| {
                        if let Some((_, timestamp)) = self.parser.get_imu(0).last() {
                            if t < (*timestamp + 1000) {
                                ui.colored_label(egui::Color32::GREEN , "0x40:IMU(B)");
                            } else {
                                ui.colored_label(egui::Color32::RED, "0x40:IMU(B)");
                            }
                        }
                        if let Some((_, timestamp)) = self.parser.get_imu(1).last() {
                            if t < *timestamp + 1000 {
                                ui.colored_label(egui::Color32::GREEN , "0x41:IMU(L)");
                            } else {
                                ui.colored_label(egui::Color32::RED, "0x41:IMU(L)");
                            }
                        }
                        if let Some((_, timestamp)) = self.parser.get_imu(2).last() {
                            if t < *timestamp + 1000 {
                                ui.colored_label(egui::Color32::GREEN , "0x42:IMU(R)");
                            } else {
                                ui.colored_label(egui::Color32::RED, "0x42:IMU(R)");
                            }
                        }
                        if let Some((_, timestamp)) = self.parser.get_imu(3).last() {
                            if t < *timestamp + 1000 {
                                ui.colored_label(egui::Color32::GREEN , "0x43:IMU(A)");
                            } else {
                                ui.colored_label(egui::Color32::RED, "0x43:IMU(A)");
                            }
                        }
                    });

                    if let Some((alt, timestamp)) = self.parser.get_ultra_sonic_data().last() {
                        if t < *timestamp + 1000 {
                            ui.colored_label(egui::Color32::GREEN , format!("0x{:02x}:UltraSonic", alt.id));
                        } else {
                            ui.colored_label(
                                egui::Color32::RED,
                                format!("0x{:02x}:UltraSonic", alt.id),
                            );
                        }
                    }

                    if let Some((gps, timestamp)) = self.parser.get_gps_data().last() {
                        if t < *timestamp + 3000 {
                            ui.colored_label(egui::Color32::GREEN , format!("0x{:02x}:GPS", gps.id));
                        } else {
                            ui.colored_label(egui::Color32::RED, format!("0x{:02x}:GPS", gps.id));
                        }
                    }

                    if let Some((vane, timestamp)) = self.parser.get_vane_data().last() {
                        if t < *timestamp + 1000 {
                            ui.colored_label(egui::Color32::GREEN , format!("0x{:02x}:Vane", vane.id));
                        } else {
                            ui.colored_label(egui::Color32::RED, format!("0x{:02x}:Vane", vane.id));
                        }
                    }

                    ui.horizontal(|ui| {
                        if let Some((_, timestamp)) = self.parser.get_barometer_data(0).last() {
                            if t < *timestamp + 1000 {
                                ui.colored_label(egui::Color32::GREEN , "0x90:Barometer (On-board)");
                            } else {
                                ui.colored_label(egui::Color32::RED, "0x90:Barometer (On-board)");
                            }
                        }
                        if let Some((_, timestamp)) = self.parser.get_barometer_data(1).last() {
                            if t < *timestamp + 1000 {
                                ui.colored_label(egui::Color32::GREEN , "0x91:Barometer (ground)");
                            } else {
                                ui.colored_label(egui::Color32::GREEN , "0x91:Barometer (ground)");
                            }
                        }
                    });
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
        let icon = include_bytes!("../assets/logo/logo.png");
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
