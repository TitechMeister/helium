use crate::parse::IMUData;
use eframe::egui;
use super::Drawable;

impl Drawable<IMUData> for IMUData {
    fn draw(data: &Vec<IMUData>, ctx: &egui::Context) {
        if let Some(imu_data) = data.last() {
            let (w, x, y, z) = (imu_data.q_w as f64 / 16384.0, imu_data.q_x as f64 / 16384.0, imu_data.q_y as f64 / 16384.0, imu_data.q_z as f64 / 16384.0);
            
            egui::Window::new(format!("IMU{:02x}",imu_data.id))
                .vscroll(true)
                .show(ctx, |ui| {
                    ui.label(format!("id:\t0x{:02x}", imu_data.id));
                    ui.heading("Quaternion:");
                    ui.label(format!("w:\t{}", w));
                    ui.label(format!("x:\t{}", x));
                    ui.label(format!("y:\t{}", y));
                    ui.label(format!("z:\t{}", z));
                    // ui.add_space(10.0);
                    // ui.heading("Euler angles:");
                    // ui.label(format!("pitch:\t{}°", euler[0].to_degrees()));
                    // ui.label(format!("roll:\t{}°", euler[1].to_degrees()));
                    // ui.label(format!("yaw:\t{}°", euler[2].to_degrees()));
                    ui.add_space(10.0);
                    ui.label(format!("timestamp:\t{}ms", imu_data.timestamp));
                });
        }
    }
}