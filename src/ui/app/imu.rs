use std::f64::consts::FRAC_PI_2;

use eframe::egui::epaint::{Color32, Stroke, Vec2};
use eframe::egui::{self, Sense};
use nalgebra::Quaternion;

use std::fs::OpenOptions;
use std::io::prelude::*;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

pub struct AppIMU {
    id: u8,
    q0: Quaternion<f64>,
    q1: Quaternion<f64>,
    invert: bool,
}

impl AppIMU {
    pub fn new(id: u8) -> Self {
        if let Ok(mut f) = OpenOptions::new()
            .read(true)
            .open(format!("log/config/imu{}_offset.bin", id))
        {
            let mut q0 = Quaternion::identity();
            let mut q1 = Quaternion::identity();
            q0.w = f.read_f64::<LittleEndian>().unwrap();
            q0.i = f.read_f64::<LittleEndian>().unwrap();
            q0.j = f.read_f64::<LittleEndian>().unwrap();
            q0.k = f.read_f64::<LittleEndian>().unwrap();
            q1.w = f.read_f64::<LittleEndian>().unwrap();
            q1.i = f.read_f64::<LittleEndian>().unwrap();
            q1.j = f.read_f64::<LittleEndian>().unwrap();
            q1.k = f.read_f64::<LittleEndian>().unwrap();
            Self {
                id: id,
                q0: q0,
                q1: q1,
                invert: false,
            }
        } else {
            Self {
                id: id,
                q0: Quaternion::identity(),
                q1: Quaternion::identity(),
                invert: false,
            }
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

        let mut config = OpenOptions::new()
            .write(true)
            .create(true)
            .open(format!("log/config/imu{}_offset.bin", self.id))
            .unwrap();
        let timestamp = chrono::Utc::now().timestamp_millis();
        file.write_all(format!("{}:{:?},{:?}\n", timestamp, self.q0, self.q1).as_bytes())
            .unwrap();
        config.write_f64::<LittleEndian>(self.q0.w).unwrap();
        config.write_f64::<LittleEndian>(self.q0.i).unwrap();
        config.write_f64::<LittleEndian>(self.q0.j).unwrap();
        config.write_f64::<LittleEndian>(self.q0.k).unwrap();
        if !self.invert {
            config.write_f64::<LittleEndian>(self.q1.w).unwrap();
            config.write_f64::<LittleEndian>(self.q1.i).unwrap();
            config.write_f64::<LittleEndian>(self.q1.j).unwrap();
            config.write_f64::<LittleEndian>(self.q1.k).unwrap();
        }else{
            config.write_f64::<LittleEndian>((Quaternion::new(0.0, 0.0, 0.0, 1.0) *self.q1).w).unwrap();
            config.write_f64::<LittleEndian>((Quaternion::new(0.0, 0.0, 0.0, 1.0) *self.q1).i).unwrap();
            config.write_f64::<LittleEndian>((Quaternion::new(0.0, 0.0, 0.0, 1.0) *self.q1).j).unwrap();
            config.write_f64::<LittleEndian>((Quaternion::new(0.0, 0.0, 0.0, 1.0) *self.q1).k).unwrap();
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

            let mut q_offset = self.q0.conjugate() * self.q1;
            if self.invert {
                q_offset = self.q0.conjugate() * Quaternion::new(0.0, 0.0, 0.0, 1.0) * self.q1;
            }

            let q = q_raw * q_offset;

            let (phi, theta, psi) = (
                (2.0 * (q.w * q.i + q.j * q.k)).atan2(1.0 - 2.0 * (q.i * q.i + q.j * q.j)),
                (-2.0 * (q.i * q.k - q.w * q.j)).asin(),
                (2.0 * (q.i * q.j + q.w * q.k)).atan2(1.0 - 2.0 * (q.j * q.j + q.k * q.k)),
            );
            egui::Window::new(format!("IMU:{:02x}", self.id))
                .resizable(true)
                .default_width(400.0)
                .vscroll(true)
                .show(ctx, |ui| {
                    egui::SidePanel::left(format!("imu_l_panel{}", self.id))
                        .resizable(true)
                        .default_width(150.0)
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
                                if ui.checkbox(&mut self.invert, "invert").changed() {
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
                    let size = ui.available_size();
                    let (response, painter) = ui.allocate_painter(size, Sense::hover());
                    let rect = response.rect;
                    let mut c = rect.center();
                    c.x = rect.min.x + rect.width() / 4.0;
                    let r = rect.width() / 4.0 - 1.0;
                    let c_pitch = c + r
                        * (-theta.sin() as f32)
                        * Vec2::new(phi.sin() as f32, phi.cos() as f32);
                    let stroke_frame = Stroke::new(1.0, Color32::DARK_GRAY);
                    painter.circle_stroke(c, r, stroke_frame);
                    painter
                        .line_segment([c - Vec2::new(r, 0.0), c + Vec2::new(r, 0.0)], stroke_frame);
                    painter
                        .line_segment([c - Vec2::new(0.0, r), c + Vec2::new(0.0, r)], stroke_frame);
                    let stroke_body = Stroke::new(1.0, Color32::BLUE);
                    painter.line_segment(
                        [
                            c_pitch
                                - r * (theta.cos() as f32)
                                    * Vec2::new(phi.cos() as f32, -phi.sin() as f32),
                            c_pitch
                                + r * (theta.cos() as f32)
                                    * Vec2::new(phi.cos() as f32, -phi.sin() as f32),
                        ],
                        stroke_body,
                    );
                });
        }
    }
}
