use eframe::egui::{self};

use super::AppUI;

pub struct TachUI {
    offset: i32,
}

impl TachUI {
    pub fn new() -> Self {
        Self {
            offset: 0,
        }
    }
}

impl AppUI for TachUI {
    fn update(&mut self, data: &mut crate::parse::Parser, ctx: &egui::Context) {
        egui::Window::new("Tachometer")
            .vscroll(true)
            .show(ctx, |ui| {
                if let Some((tach_data,_)) = data.get_tach_data(1).last() {
                    ui.heading(format!("Cadence:\t{}", tach_data.cadence));
                    // ui.heading(format!("Strain:\t{}"));
                    if data.get_tach_data(1).len() >= 10{
                        ui.label(format!("Strain:\t{}", tach_data.strain - self.offset));
                    } else {
                        ui.label("Strain:\tN/A");
                    }
                    ui.add_space(10.0);
                    ui.label(format!("timestamp:\t{}ms", tach_data.timestamp));
                    if ui.button("Set offset").clicked() {
                        self.offset = tach_data.strain;
                    }
                    
                    egui_plot::Plot::new("tach_plot")
                        .show(ui, |plot_ui| {
                            /*
                            let circle_points: PlotPoints = (0..512)
                                .map(|i| {
                                    let theta = PI * (i as f64) / 512.0;
                                    [theta.cos(), theta.sin()]
                                })
                                .collect();
                            let theta = (1.0-tach_data.cadence as f64 / 180.0) * PI;
                            */

                            if data.get_tach_data(1).len() > 100 {
                                let point_strain: egui_plot::PlotPoints = data
                                    .get_tach_data(1)[data.get_tach_data(1).len() - 100..]
                                    .iter()
                                    .map(|(data, time)| [*time as f64, data.strain as f64])
                                    .collect();

                                plot_ui.line(
                                    egui_plot::Line::new(point_strain)
                                        .color(egui::Color32::from_rgb(0, 64, 255))
                                        .name("ultra sonic")
                                        .fill(0.0),
                                );
                            }

                        });
                }
            });
    }
}
