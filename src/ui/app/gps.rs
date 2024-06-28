use eframe::egui;

pub struct Gps {
}

impl Gps {
    pub fn new() -> Self {
        Self {}
    }
}

impl super::AppUI for Gps {
    fn update(&mut self, data: &crate::parse::Parser, ctx: &eframe::egui::Context) {
        egui::Window::new(format!("GPS"))
            .show(ctx, |ui| {
                if let Some(gps_data) = data.get_gps_data().last() {
                    ui.heading(format!("lon:\t{}", gps_data.longitude));
                    ui.heading(format!("lat:\t{}", gps_data.latitude));
                    ui.add_space(10.0);
                    ui.label(format!("timestamp:\t{}ms", gps_data.timestamp));
                }
            });
    }
}
