use eframe::egui;

pub struct PitotUI {}

impl PitotUI {
    pub fn new() -> Self {
        Self {}
    }
}

impl super::AppUI for PitotUI {
    fn update(&mut self, data: &mut crate::parse::Parser, ctx: &eframe::egui::Context) {
        if let Some(pitot_data) = data.get_pitot_data().last() {
            egui::Window::new("Pitot").vscroll(true).show(ctx, |ui| {
                ui.heading(format!(
                    "velocity:\t{:2.2}m/s\ttimestamp:\t{}ms",
                    pitot_data.velocity, pitot_data.timestamp
                ));
                let plt = egui_plot::Plot::new("velocity")
                    .allow_zoom(false)
                    .allow_scroll(false);
                let point: egui_plot::PlotPoints = data.get_pitot_data()
                    .iter()
                    .enumerate()
                    .map(|(n, _data)| [n as f64, _data.velocity as f64])
                    .collect();
                let line = egui_plot::Line::new(point)
                    .color(egui::Color32::from_rgb(255, 0, 0))
                    .name("velocity")
                    .fill(0.0);
                plt.show(ui, |plot_ui| {
                    plot_ui.line(line);
                });
            });
        }
    }
}