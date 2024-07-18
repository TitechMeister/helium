use eframe::egui;

use super::AppUI;

pub struct AltitudeUI {}

impl AltitudeUI {
    pub fn new() -> Self {
        Self {}
    }
}

impl AppUI for AltitudeUI {
    fn update(&mut self, data: &mut crate::parse::Parser, ctx: &eframe::egui::Context) {
        if let Some(alt_data) = data.get_ultra_sonic_data().last() {
            egui::Window::new("Altitude").vscroll(true).show(ctx, |ui| {
                ui.heading(format!(
                    "altitude:\t{:2.2}m\ttimestamp:\t{}ms",
                    alt_data.altitude / 100.0,
                    alt_data.timestamp
                ));
                egui_plot::Plot::new("Altitude")
                    .allow_zoom(false)
                    .allow_scroll(false)
                    .show(ui, |ui| {
                        let point: egui_plot::PlotPoints = data
                            .get_ultra_sonic_data()
                            .iter()
                            .enumerate()
                            .map(|(n, _data)| [n as f64, _data.altitude as f64 / 100.0])
                            .collect();
                        let line = egui_plot::Line::new(point)
                            .color(egui::Color32::from_rgb(0, 64, 255))
                            .name("altitude")
                            .fill(0.0);
                        // find the max altitude
                        let altitude: Vec<f32> =
                            data.get_ultra_sonic_data().iter().map(|x| x.altitude).collect();
                        let max_altitude: f32 = altitude.iter().fold(0.0, |a, &b| a.max(b));

                        ui.set_plot_bounds(egui_plot::PlotBounds::from_min_max(
                            [0.0, 0.0],
                            [100.0, max_altitude as f64 / 100.0],
                        ));
                        ui.line(line);
                    });
            });
        }
    }
}
