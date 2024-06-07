use crate::parse::VaneData;
use eframe::egui;
use super::Drawable;

impl Drawable<VaneData> for VaneData {
    fn draw(data: &Vec<VaneData>, ctx: &egui::Context) {
        if let Some(vane_data) = data.last() {
            egui::Window::new(format!("Vane{:02x}",vane_data.id))
                .vscroll(true)
                .show(ctx, |ui| {
                    ui.heading(format!("angle:\t{}", vane_data.angle));
                    ui.add_space(10.0);
                    ui.label(format!("timestamp:\t{}ms", vane_data.timestamp));
                });
        }
    }
}