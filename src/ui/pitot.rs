use eframe::egui;

pub struct PitotUI {}

impl PitotUI {
    pub fn new() -> Self {
        Self {}
    }
}

impl super::AppUI for PitotUI {
    fn update(&mut self, data: &mut crate::parse::Parser, ctx: &eframe::egui::Context) {
        if let Some((pitot_data, _)) = data.get_pitot_data().last() {
            egui::Window::new("Pitot").vscroll(true).show(ctx, |ui| {
                ui.heading(format!(
                    "IAS:\t{:2.2}m/s\ttimestamp:\t{}ms",
                    pitot_data.velocity, pitot_data.timestamp
                ));
                egui_plot::Plot::new("velocity")
                    .legend(egui_plot::Legend::default())
                    .show(ui, |plt_ui| {
                        let point_ias: egui_plot::PlotPoints = data
                            .get_pitot_data()
                            .iter()
                            .map(|(_data, utc)| [*utc as f64, _data.velocity as f64])
                            .collect();

                        let point_cas: egui_plot::PlotPoints = data
                            .get_pitot_data()
                            .iter()
                            .map(|(pitot_data, pitot_timestamp)| {

                                let c3 = 2.0e-5;
                                let c2 = 0.0032;
                                let c1 = 1.0073;
                                let c0 = 1.0073;

                                let mut c = c0;

                                if let Some((vane_data, _)) = data
                                    .get_vane_data()
                                    .iter()
                                    .rfind(|(_, vane_timestamp)| vane_timestamp <= pitot_timestamp)
                                {
                                    c =   c0
                                        + c1 * vane_data.angle
                                        + c2 * vane_data.angle * vane_data.angle
                                        + c3 * vane_data.angle * vane_data.angle * vane_data.angle;
                                }
                                [*pitot_timestamp as f64, (c * pitot_data.velocity) as f64]
                            })
                            .collect();
                        plt_ui.line(
                            egui_plot::Line::new(point_ias)
                                .color(egui::Color32::from_rgb(255, 0, 0))
                                .name("IAS")
                                .fill(0.0),
                        );
                        plt_ui.line(
                            egui_plot::Line::new(point_cas)
                                .color(egui::Color32::from_rgb(0, 255, 0))
                                .name("CAS")
                                .fill(0.0),
                        );
                    });
            });
        }
    }
}
