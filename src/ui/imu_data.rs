use crate::parse::IMUData;
use eframe::egui;
use super::Drawable;

impl Drawable<IMUData> for IMUData {
    fn draw(data: &Vec<IMUData>, ctx: &egui::Context) {
        if let Some(imu_data) = data.last() {
            let (w, x, y, z) = (imu_data.q_w as f64 / 16384.0, imu_data.q_x as f64 / 16384.0, imu_data.q_y as f64 / 16384.0, imu_data.q_z as f64 / 16384.0);
            let c = {
                let (w, x, y, z) = (w, x, y, z);
                let (w2, x2, y2, z2) = (w*w, x*x, y*y, z*z);
                let (wx, wy, wz, xy, xz, yz) = (w*x, w*y, w*z, x*y, x*z, y*z);
                [
                    [w2 + x2 - y2 - z2, 2.0*(xy + wz), 2.0*(xz-wy)],
                    [2.0*(xy - wz), w2 - x2 + y2 - z2, 2.0*(yz + wx)],
                    [2.0*(xz + wy), 2.0*(yz - wx), w2 - x2 - y2 + z2]
                ]
            };
            assert!(c[0][0]*c[0][0] + c[0][1]*c[0][1] + c[0][2]*c[0][2] - 1.0 < 1e-6);
            assert!(c[1][0]*c[1][0] + c[1][1]*c[1][1] + c[1][2]*c[1][2] - 1.0 < 1e-6);
            assert!(c[2][0]*c[2][0] + c[2][1]*c[2][1] + c[2][2]*c[2][2] - 1.0 < 1e-6);
            // zyx euler angles
            let euler = {
                let pitch = (c[2][0] as f64).atan2(-c[2][1] as f64);
                let roll = (c[2][2] as f64).asin();
                let yaw = (c[0][2] as f64).atan2(c[1][2] as f64);
                [pitch, roll, yaw]
            };
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
                    ui.label(format!("pitch:\t{}°", euler[0].to_degrees()));
                    ui.label(format!("roll:\t{}°", euler[1].to_degrees()));
                    ui.label(format!("yaw:\t{}°", euler[2].to_degrees()));
                    ui.add_space(10.0);
                    ui.label(format!("timestamp:\t{}ms", imu_data.timestamp));
                });
        }
    }
}