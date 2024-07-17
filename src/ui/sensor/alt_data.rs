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
                let plt = egui_plot::Plot::new("Altitude")
                    .allow_zoom(false)
                    .allow_scroll(false);
                let point: egui_plot::PlotPoints = data
                    .iter()
                    .enumerate()
                    .map(|(n, _data)| [n as f64, _data.altitude as f64 / 100.0])
                    .collect();
                let line = egui_plot::Line::new(point)
                    .color(egui::Color32::from_rgb(0, 64, 255))
                    .name("altitude")
                    .fill(0.0);
                // find the max altitude
                let altitude:Vec<f32> = data.iter().map(|x| x.altitude).collect();
                let max_altitude:f32 = altitude.iter().fold(0.0, |a, &b| a.max(b));

                plt.show(ui, |plot_ui| {
                    plot_ui.set_plot_bounds(
                        egui_plot::PlotBounds::from_min_max([0.0,0.0], [100.0,max_altitude as f64 / 100.0])
                    );
                    plot_ui.line(line);
                });
            });
        }
    }
}