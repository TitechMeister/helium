use super::Drawable;
use crate::parse::BarometerData;
use eframe::egui;

impl Drawable<BarometerData> for BarometerData {
    fn draw(data: &Vec<BarometerData>, ctx: &egui::Context) {
        if let Some(alt_data) = data.last() {
            egui::Window::new("Pressure").vscroll(true).show(ctx, |ui| {
                ui.heading(format!(
                    "pressure:\t{:2.2}Pa\ttimestamp:\t{}ms",
                    alt_data.pressure, alt_data.timestamp
                ));
                let plt = egui_plot::Plot::new("Pressure")
                    .allow_zoom(false)
                    .allow_scroll(false);
                let point: egui_plot::PlotPoints = data
                    .iter()
                    .enumerate()
                    .map(|(n, _data)| [n as f64, _data.pressure as f64])
                    .collect();
                let line = egui_plot::Line::new(point)
                    .color(egui::Color32::from_rgb(0, 255, 64))
                    .name("pressure");
                plt.show(ui, |plot_ui| {
                    plot_ui.line(line);
                });
            });
        }
    }
}
