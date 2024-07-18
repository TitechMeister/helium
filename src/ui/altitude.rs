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
        egui::Window::new("Altitude").vscroll(true).show(ctx, |ui| {
            if let Some(alt_data) = data.get_ultra_sonic_data().last() {
                ui.heading(format!(
                    "altitude:\t{:2.2}m\ttimestamp:\t{}ms",
                    alt_data.altitude / 100.0,
                    alt_data.timestamp
                ));
            }
            
            let mut p0 = 101300.0;
            if let Some(barometer_data) = data.get_barometer_data(1).last() {
                p0 = barometer_data.pressure;

                if let Some(barometer_data0)= data.get_barometer_data(0).last(){
                    ui.label(
                        format!(
                            "Pressure: {:2.2}Pa\tTemperature: {:2.2}C",
                            barometer_data0.pressure, barometer_data0.temperature
                        )
                    );
                    ui.heading(
                        format!(
                            "altitude:\t{:2.2}m\ttimestamp:\t{}ms",
                            44330.0 * (1.0 - (barometer_data0.pressure / p0).powf(1.0 / 5.255)),
                            barometer_data0.timestamp
                        )
                    );
                }
            }

            egui_plot::Plot::new("Altitude")
                .show(ui, |plt_ui| {
                    let point_ultra_sonic: egui_plot::PlotPoints = data
                        .get_ultra_sonic_data()
                        .iter()
                        .enumerate()
                        .map(|(n, _data)| [n as f64, _data.altitude as f64 / 100.0])
                        .collect();

                    let point_barometer: egui_plot::PlotPoints = data
                        .get_barometer_data(0)
                        .iter()
                        .enumerate()
                        .map(|(n, _data)| {
                            [
                                n as f64,
                                44330.0 * (1.0 - (_data.pressure / p0).powf(1.0 / 5.255)) as f64,
                            ]
                        })
                        .collect();

                    plt_ui.line(
                        egui_plot::Line::new(point_ultra_sonic)
                            .color(egui::Color32::from_rgb(0, 64, 255))
                            .name("ultra sonic")
                            .fill(0.0),
                    );

                    plt_ui.line(
                        egui_plot::Line::new(point_barometer)
                            .color(egui::Color32::from_rgb(255, 64, 0))
                            .name("barometer")
                            .fill(0.0),
                    );
                });
        });
    }
}
