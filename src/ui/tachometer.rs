use std::f64::consts::PI;

use eframe::egui::{self, Color32};
use egui_plot::{Line, PlotPoints};

use super::AppUI;

pub struct TachUI {}

impl TachUI {
    pub fn new() -> Self {
        Self {}
    }
}

impl AppUI for TachUI {
    fn update(&mut self, data: &mut crate::parse::Parser, ctx: &egui::Context) {
        egui::Window::new("Tachometer")
            .vscroll(true)
            .show(ctx, |ui| {
                if let Some((tach_data,_)) = data.get_tach_data(1).last() {
                    ui.heading(format!("Cadence:\t{}", tach_data.cadence));
                    ui.add_space(10.0);
                    ui.label(format!("timestamp:\t{}ms", tach_data.timestamp));
                    egui_plot::Plot::new("tach_plot")
                        .data_aspect(1.0)
                        .show(ui, |plot_ui| {
                            let circle_points: PlotPoints = (0..512)
                                .map(|i| {
                                    let theta = PI * (i as f64) / 512.0;
                                    [theta.cos(), theta.sin()]
                                })
                                .collect();
                            let theta = (1.0-tach_data.cadence as f64 / 180.0) * PI;

                            plot_ui.line(
                                Line::new(circle_points).color(Color32::from_rgb(127, 127, 127)),
                            );
                            plot_ui.line(
                                Line::new(PlotPoints::new(vec![
                                    [0.0, 0.0],
                                    [theta.cos(), theta.sin()],
                                ]))
                                .color(Color32::from_rgb(255, 0, 0)),
                            );
                        });
                }
            });
    }
}
