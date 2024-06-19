use std::f64::consts::{FRAC_PI_2, PI};

use eframe::egui;
use egui_plot;
use nalgebra::Quaternion;

use std::fs::OpenOptions;
use std::io::prelude::*;

pub struct AppIMU {
    id: u8,
    q0: Quaternion<f64>,
    q1: Quaternion<f64>,
    invert: bool,
}

impl AppIMU {
    pub fn new(id: u8) -> Self {
        Self {
            id: id,
            q0: Quaternion::identity(),
            q1: Quaternion::identity(),
            invert: false,
        }
    }
    pub fn save_offset(&mut self) {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(format!(
                "log/{}/imu{}_offset.txt",
                chrono::Local::now().format("%m%d"),
                self.id
            ))
            .unwrap();
        let timestamp = chrono::Utc::now().timestamp_millis();
        if self.invert{
            file.write_all(format!("{}:{:?}\n", timestamp, self.q0.conjugate() * self.q1).as_bytes())
            .unwrap();
        }else{
            file.write_all(format!("{}:{:?}\n", timestamp, self.q0.conjugate() * Quaternion::new(0.0, 0.0, 0.0, 1.0) * self.q1).as_bytes())
            .unwrap();
        }
        
    }
}

impl super::AppUI for AppIMU {
    fn update(&mut self, data: &crate::parse::Parser, ctx: &eframe::egui::Context) {
        if let Some(imu) = data.get_imu(self.id).last() {
            let q_raw = Quaternion::new(
                imu.q_w as f64 / 16384.0,
                imu.q_x as f64 / 16384.0,
                imu.q_y as f64 / 16384.0,
                imu.q_z as f64 / 16384.0,
            );

            let mut q = q_raw * self.q0.conjugate() * self.q1;

            if self.invert {
                q = q_raw * self.q0.conjugate() * Quaternion::new(0.0, 0.0, 0.0, 1.0) * self.q1;
            }
            let (phi, theta, psi) = (
                (2.0 * (q.w * q.i + q.j * q.k)).atan2(1.0 - 2.0 * (q.i * q.i + q.j * q.j)),
                (-2.0 * (q.i * q.k - q.w * q.j)).asin(),
                (2.0 * (q.i * q.j + q.w * q.k)).atan2(1.0 - 2.0 * (q.j * q.j + q.k * q.k)),
            );
            egui::Window::new(format!("IMU:{:02x}", self.id))
                .vscroll(true)
                .show(ctx, |ui| {
                    egui::SidePanel::left("imu_l_panel")
                        .resizable(true)
                        .default_width(250.0)
                        .show_inside(ui, |ui| {
                            ui.heading("Euler angles:");
                            ui.label(format!("roll:\t{:4.3}°", phi.to_degrees()));
                            ui.label(format!("pitch:\t{:4.3}°", theta.to_degrees()));
                            ui.label(format!("yaw:\t{:4.3}°", psi.to_degrees()));

                            ui.add_space(10.0);

                            ui.collapsing("Setting", |ui| {
                                if ui.button("detect q0").clicked() {
                                    self.q0 = q_raw;
                                    self.q1 = Quaternion::identity();
                                }
                                if ui.button("detect x axis").clicked() {
                                    let rotq = q_raw * self.q0.conjugate();
                                    let psi0 = -rotq.j.atan2(rotq.i);
                                    self.q1 = Quaternion::exp(&Quaternion::new(
                                        0.0,
                                        0.0,
                                        0.0,
                                        -psi0 / 2.0,
                                    ));
                                    self.save_offset();
                                }
                                if ui.button("detect y axis").clicked() {
                                    let rotq = q_raw * self.q0.conjugate();
                                    let psi0 = -rotq.j.atan2(rotq.i) + FRAC_PI_2;
                                    self.q1 = Quaternion::exp(&Quaternion::new(
                                        0.0,
                                        0.0,
                                        0.0,
                                        -psi0 / 2.0,
                                    ));
                                    self.save_offset();
                                }
                                if ui.checkbox(&mut self.invert, "invert").changed(){
                                    self.save_offset();
                                }
                                ui.add_space(10.0);
                                let q_c = q_raw.conjugate() * q; // correction quaternion
                                ui.label("correction quaternion is");
                                ui.add_space(5.0);
                                ui.label(format!(
                                    "{:1.3},\n{:1.3},\n{:1.3},\n{:1.3}",
                                    q_c.w, q_c.i, q_c.j, q_c.k
                                ));
                            });

                            ui.collapsing("Status", |ui| {
                                ui.label(format!("timestamp:\t{} ms", imu.timestamp));
                                ui.add_space(10.0);
                                ui.label("Calibration Status");
                                ui.label(format!("\tacc : {}/3", (imu.calib & 0xF000) / 4096));
                                ui.label(format!("\tgyr : {}/3", (imu.calib & 0x0F00) / 256));
                                ui.label(format!("\tmag : {}/3", (imu.calib & 0x00F0) / 16));
                                ui.label(format!("\tsys : {}/3", (imu.calib & 0x000F)));
                            });
                        });

                    let plt = egui_plot::Plot::new("imu").data_aspect(1.0);
                    let pt_c_roll: egui_plot::PlotPoints = (0..512)
                        .map(|i| {
                            let theta = i as f64 / 512.0 * 2.0 * PI;
                            [theta.cos() - 1.0, theta.sin()]
                        })
                        .collect();
                    let line_c_roll =
                        egui_plot::Line::new(pt_c_roll).color(egui::Color32::from_rgb(0, 127, 127));
                    let line_roll = egui_plot::Line::new(egui_plot::PlotPoints::new(vec![
                        [-phi.cos() - 1.0, -phi.sin()],
                        [phi.cos() - 1.0, phi.sin()],
                    ]))
                    .color(egui::Color32::from_rgb(0, 127, 255));

                    let pt_c_yaw: egui_plot::PlotPoints = (0..512)
                        .map(|i| {
                            let theta = i as f64 / 512.0 * 2.0 * PI;
                            [theta.cos() + 1.0, theta.sin()]
                        })
                        .collect();
                    let line_c_yaw =
                        egui_plot::Line::new(pt_c_yaw).color(egui::Color32::from_rgb(255, 127, 0));
                    let line_yaw = egui_plot::Line::new(egui_plot::PlotPoints::new(vec![
                        [1.0, 0.0],
                        [-psi.sin() + 1.0, psi.cos()],
                    ]))
                    .color(egui::Color32::from_rgb(0, 127, 255));
                    plt.show(ui, |plt_ui| {
                        plt_ui.line(line_c_yaw);
                        plt_ui.line(line_yaw);
                        plt_ui.line(line_c_roll);
                        plt_ui.line(line_roll);
                    });
                });
        }
    }
}
