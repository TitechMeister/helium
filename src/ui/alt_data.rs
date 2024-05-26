use crate::parse::AltData;
use eframe::egui;
use super::Drawable;

impl Drawable<AltData> for AltData {
    fn draw(data: &Vec<AltData>, ctx: &egui::Context) {
        if let Some(alt_data) = data.last() {
            egui::Window::new("Altitude").vscroll(true).show(ctx, |ui| {
                ui.heading(format!(
                    "altitude:\t{:2.2}m\ttimestamp:\t{}ms",
                    alt_data.altitude / 100.0,
                    alt_data.timestamp
                ));
                let plt = egui_plot::Plot::new("Altitude");
                let point: egui_plot::PlotPoints = data
                    .iter()
                    .enumerate()
                    .map(|(n, _data)| [n as f64, _data.altitude as f64 / 100.0])
                    .collect();
                let line = egui_plot::Line::new(point)
                    .color(egui::Color32::from_rgb(0, 64, 255))
                    .name("altitude");
                plt.show(ui, |plot_ui| {
                    plot_ui.line(line);
                });
            });
        }
    }
}