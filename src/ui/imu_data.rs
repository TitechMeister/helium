use crate::parse::IMUData;
use eframe::egui;
use super::Drawable;

impl Drawable<IMUData> for IMUData {
    fn draw(data: &Vec<IMUData>, ctx: &egui::Context) {
        if let Some(imu_data) = data.last() {
            let (w, x, y, z) = (imu_data.q_w as f64 / 16384.0, imu_data.q_x as f64 / 16384.0, imu_data.q_y as f64 / 16384.0, imu_data.q_z as f64 / 16384.0);
            let (phi,theta,psi)=(
                (2.0*(w*x+y*z)/(w*w-x*x-y*y+z*z)).atan(),
                (-2.0*(x*z-w*y)).asin(),
                (2.0*(x*y+w*z)/(w*w+x*x-y*y-z*z)).atan()
            );
            
            egui::Window::new(format!("IMU{:02x}",imu_data.id))
                .vscroll(true)
                .show(ctx, |ui| {
                    ui.label(format!("id:\t0x{:02x}", imu_data.id));
                    ui.heading("Quaternion:");
                    ui.label(format!("w:\t{}", w));
                    ui.label(format!("x:\t{}", x));
                    ui.label(format!("y:\t{}", y));
                    ui.label(format!("z:\t{}", z));
                    ui.add_space(10.0);
                    ui.heading("Euler angles:");
                    ui.label(format!("pitch:\t{}°", theta.to_degrees()));
                    ui.label(format!("roll:\t{}°", phi.to_degrees()));
                    ui.label(format!("yaw:\t{}°", psi.to_degrees()));
                    ui.add_space(10.0);
                    ui.label(format!("calib(acc,gyr,mag,sys):{:04x}",imu_data.calib));
                    ui.label(format!("timestamp:\t{}ms", imu_data.timestamp));
                });
        }
    }
}