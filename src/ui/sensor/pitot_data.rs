use crate::parse::PitotData;
use eframe::egui;
use super::Drawable;

impl Drawable<PitotData> for PitotData {
    fn draw(data: &Vec<PitotData>, ctx: &egui::Context) {
        if let Some(pitot_data) = data.last() {
            egui::Window::new("Pitot").vscroll(true).show(ctx, |ui| {
                ui.heading(format!(
                    "velocity:\t{:2.2}m/s\ttimestamp:\t{}ms",
                    pitot_data.velocity, pitot_data.timestamp
                ));
                let plt = egui_plot::Plot::new("velocity")
                    .allow_zoom(false)
                    .allow_scroll(false);
                let point: egui_plot::PlotPoints = data
                    .iter()
                    .enumerate()
                    .map(|(n, _data)| [n as f64, _data.velocity as f64])
                    .collect();
                let line = egui_plot::Line::new(point)
                    .color(egui::Color32::from_rgb(255, 0, 0))
                    .name("velocity");
                plt.show(ui, |plot_ui| {
                    plot_ui.line(line);
                });
            });
        }
    }
}