use eframe::egui;

use super::AppUI;

pub struct PowerEstimaterUI {
    visible: bool,
}

impl PowerEstimaterUI {
    pub fn new() -> Self {
        Self { visible: true }
    }
}

impl AppUI for PowerEstimaterUI {
    fn update(&mut self, data: &mut crate::parse::Parser, ctx: &eframe::egui::Context) {
        egui::Window::new("PowerEstimater")
            .vscroll(true)
            .show(ctx, |ui| {
                let link_group_id = ui.id().with("power_est");
                ui.checkbox(&mut self.visible, "visible");
                if self.visible {
                    ui.horizontal(|ui| {
                        //Pitot

                        egui_plot::Plot::new("velocity")
                            .legend(egui_plot::Legend::default())
                            .link_axis(link_group_id, true, false)
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
                                            .rfind(|(_, vane_timestamp)| {
                                                vane_timestamp <= pitot_timestamp
                                            })
                                        {
                                            c = c0
                                                + c1 * vane_data.angle
                                                + c2 * vane_data.angle * vane_data.angle
                                                + c3 * vane_data.angle
                                                    * vane_data.angle
                                                    * vane_data.angle;
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

                        // Alt

                        egui_plot::Plot::new("pwr_esr_altitude")
                            .legend(egui_plot::Legend::default())
                            .show(ui, |plt_ui| {
                                if data.get_ultra_sonic_data().len() > 100 {
                                    let point_ultra_sonic: egui_plot::PlotPoints = data
                                        .get_ultra_sonic_data()
                                        [data.get_ultra_sonic_data().len() - 100..]
                                        .iter()
                                        .map(|(data, time)| [*time as f64, data.altitude as f64])
                                        .collect();

                                    plt_ui.line(
                                        egui_plot::Line::new(point_ultra_sonic)
                                            .color(egui::Color32::from_rgb(0, 64, 255))
                                            .name("ultra sonic")
                                            .fill(0.0),
                                    );
                                }
                                if data.get_barometer_data(0).len() > 100 {
                                    let p0=101300.0;
                                    let point_barometer: egui_plot::PlotPoints = data
                                        .get_barometer_data(0)
                                        .iter()
                                        .map(|(baro0_data, baro0_time)| {
                                            if let Some((baro1_data, _)) = data
                                                .get_barometer_data(1)
                                                .iter()
                                                .rfind(|(_, baro1_time)| baro1_time <= baro0_time)
                                            {
                                                [
                                                    *baro0_time as f64,
                                                    44330.0
                                                        * (1.0
                                                            - (baro0_data.pressure
                                                                / baro1_data.pressure)
                                                                .powf(1.0 / 5.255))
                                                            as f64
                                                        + (1.2 as f64),
                                                ]
                                            } else {
                                                [
                                                    *baro0_time as f64,
                                                    44330.0
                                                        * (1.0
                                                            - (baro0_data.pressure / p0)
                                                                .powf(1.0 / 5.255))
                                                            as f64,
                                                ]
                                            }
                                        })
                                        .collect();
                                    plt_ui.line(
                                        egui_plot::Line::new(point_barometer)
                                            .color(egui::Color32::from_rgb(255, 64, 0))
                                            .name("barometer")
                                            .fill(0.0),
                                    );
                                }
                            });

                        // Tachometer

                        egui_plot::Plot::new("power_est_tach")
                            .legend(egui_plot::Legend::default())
                            .link_axis(link_group_id, true, false)
                            .show(ui, |plt_ui| {
                                let point_cadence: egui_plot::PlotPoints = data
                                    .get_tach_data(1)
                                    .iter()
                                    .map(|(_data, utc)| [*utc as f64, _data.cadence as f64])
                                    .collect();
                                plt_ui.line(
                                    egui_plot::Line::new(point_cadence)
                                        .color(egui::Color32::from_rgb(255, 0, 0))
                                        .name("cadence")
                                        .fill(0.0),
                                );
                            });

                    });
                }
            });
    }
}
