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
        if let Some(vane_data) = data.get_tach_data(1).last() {
            egui::Window::new(format!("Tachometer{:02x}", vane_data.id))
                .vscroll(true)
                .show(ctx, |ui| {
                    ui.heading(format!("Cadence:\t{}", vane_data.cadence));
                    ui.add_space(10.0);
                    ui.label(format!("timestamp:\t{}ms", vane_data.timestamp));
                    let plt = egui_plot::Plot::new("tach_plot")
                        .data_aspect(1.0)
                        .allow_zoom(false)
                        .allow_scroll(false);
                    let theta = (vane_data.cadence as f64 / 180.0 ) * PI;
                    let circle_points: PlotPoints = (0..512)
                        .map(|i| {
                            let theta = 2.0 * PI * (i as f64) / 512.0;
                            [theta.cos(), theta.sin()]
                        })
                        .collect();
                    let circle_line = Line::new(circle_points)
                        .color(Color32::from_rgb(127, 127, 127))
                        .name("tach_circle");
                    let arrow_line = Line::new(PlotPoints::new(vec![
                        [0.0, 0.0],
                        [theta.cos(), theta.sin()],
                    ]))
                    .color(Color32::from_rgb(255, 0, 0));
                    plt.show(ui, |plot_ui| {
                        plot_ui.line(circle_line);
                        plot_ui.line(arrow_line);
                    });
                });
        }
    }
}
