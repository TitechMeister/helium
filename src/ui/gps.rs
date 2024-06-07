use crate::parse::GPSData;
use eframe::egui;
use super::Drawable;

impl Drawable<GPSData> for GPSData {
    fn draw(data: &Vec<GPSData>, ctx: &egui::Context) {
        if let Some(imu_data) = data.last() {
            egui::Window::new(format!("GPS{:02x}",imu_data.id))
                .vscroll(true)
                .show(ctx, |ui| {
                    ui.heading(format!("lon:\t{}", imu_data.longitude));
                    ui.heading(format!("lat:\t{}", imu_data.latitude));
                    ui.add_space(10.0);
                    ui.label(format!("timestamp:\t{}ms", imu_data.timestamp));
                });
        }
    }
}