use std::f64::consts::PI;

use eframe::egui::{self, Color32};
use egui_plot::{Line, PlotPoints};

use super::AppUI;

pub struct VaneUI {}

impl VaneUI {
    pub fn new() -> Self {
        Self {}
    }
}

impl AppUI for VaneUI {
    fn update(&mut self, data: &mut crate::parse::Parser, ctx: &egui::Context) {
        if let Some((vane_data,_)) = data.get_vane_data().last() {
            egui::Window::new(format!("Vane{:02x}", vane_data.id))
                .vscroll(true)
                .show(ctx, |ui| {
                    ui.heading(format!("angle:\t{}", vane_data.angle));
                    ui.add_space(10.0);
                    ui.label(format!("timestamp:\t{}ms", vane_data.timestamp));
                    let plt = egui_plot::Plot::new("vane_angle")
                        .data_aspect(1.0)
                        .allow_zoom(false)
                        .allow_scroll(false);
                    let theta = (-vane_data.angle as f64 / 180.0 + 0.5) * PI;
                    let circle_points: PlotPoints = (0..512)
                        .map(|i| {
                            let theta = 2.0 * PI * (i as f64) / 512.0;
                            [theta.cos(), theta.sin()]
                        })
                        .collect();
                    let circle_line = Line::new(circle_points)
                        .color(Color32::from_rgb(127, 127, 127))
                        .name("vane_circle");
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
