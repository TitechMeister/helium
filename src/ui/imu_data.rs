use crate::parse::IMUData;
use eframe::egui;
use super::Drawable;

impl Drawable<IMUData> for IMUData {
    fn draw(data: &Vec<IMUData>, ctx: &egui::Context) {
        if let Some(imu_data) = data.last() {
            egui::Window::new(format!("IMU{:02x}",imu_data.id))
                .vscroll(true)
                .show(ctx, |ui| {
                    ui.heading(format!("timestamp:\t{}ms", imu_data.timestamp));
                    ui.heading(format!("q_w:\t{}", imu_data.q_w));
                    ui.heading(format!("q_x:\t{}", imu_data.q_x));
                    ui.heading(format!("q_y:\t{}", imu_data.q_y));
                    ui.heading(format!("q_z:\t{}", imu_data.q_z));
                });
        }
    }
}