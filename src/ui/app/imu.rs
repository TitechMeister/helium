use std::f64::consts::PI;

use eframe::egui;
use egui_plot;
use nalgebra::Quaternion;

pub struct AppIMU {
    q0: Quaternion<f64>,
    q1:Quaternion<f64>,
    invert: bool
}

impl Default for AppIMU {
    fn default() -> Self {
        Self {
            q0: Quaternion::new(1.0, 0.0, 0.0, 0.0),
            q1: Quaternion::new(1.0, 0.0, 0.0, 0.0),
            invert:false
        }
    }
}

impl super::AppUI for AppIMU {
    fn update(&mut self, data: &crate::parse::Parser, ctx: &eframe::egui::Context) {
        if let Some(imu) = data.get_imu(0x40).last() {
            let q_raw = Quaternion::new(
                imu.q_w as f64 / 16384.0,
                imu.q_x as f64 / 16384.0,
                imu.q_y as f64 / 16384.0,
                imu.q_z as f64 / 16384.0,
            );
            
            let mut q = q_raw*(self.q0.conjugate())*self.q1;

            if self.invert {
                q=q_raw*self.q0.conjugate()*Quaternion::new(0.0,0.0,0.0,1.0)*self.q1;
            }
            let (phi, theta, mut psi) = (
                (2.0 * (q.w * q.i + q.j * q.k).atan2(1.0 - 2.0 * (q.i * q.i + q.j * q.j))),
                (-2.0 * (q.i * q.k - q.w * q.j)).asin(),
                (2.0 * (q.i * q.j + q.w * q.k).atan2(1.0 - 2.0 * (q.j * q.j + q.k * q.k))),
            );
            egui::Window::new("IMU").vscroll(true).show(ctx, |ui| {
                egui::SidePanel::left("imu_l_panel")
                    .resizable(true)
                    .default_width(150.0)
                    .show_inside(ui, |ui| {
                        ui.heading("Euler angles:");
                        ui.label(format!("roll:\t{:3.3}°", phi.to_degrees()));
                        ui.label(format!("pitch:\t{:3.3}°", theta.to_degrees()));
                        ui.label(format!("yaw:\t{:3.3}°", psi.to_degrees()));

                        ui.add_space(10.0);

                        ui.collapsing("setting", |ui| {
                            if ui.button("detect q0").clicked() {
                                self.q0 = q_raw;
                                self.q1 = Quaternion::new(1.0, 0.0, 0.0, 0.0);
                            }
                            if ui.button("detect psi1").clicked() {
                                let rotq = self.q0.conjugate() * q_raw;
                                let psi0 = rotq.j.atan2(rotq.i);
                                self.q1 = Quaternion::exp(&Quaternion::new(0.0, 0.0, 0.0, psi0/2.0)).conjugate();
                            }
                            ui.checkbox(&mut self.invert, "invert");
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
                if self.invert{
                    psi+=PI;
                }
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
