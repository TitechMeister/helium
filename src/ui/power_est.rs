use eframe::egui;

use super::AppUI;

pub struct PowerEstimaterUI {}

impl PowerEstimaterUI {
    pub fn new() -> Self {
        Self {}
    }
}

impl AppUI for PowerEstimaterUI {
    fn update(&mut self, data: &mut crate::parse::Parser, ctx: &eframe::egui::Context) {
        egui::Window::new("PowerEstimater").vscroll(true).show(ctx, |ui| {
            let link_group_id=ui.id().with("power_est");
            ui.horizontal(|ui|{
                Plot::new("PowerEstimate")
            });
        });
    }
}
