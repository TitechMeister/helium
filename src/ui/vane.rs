use std::f64::consts::PI;

use crate::parse::VaneData;
use eframe::egui::{self,Color32};
use egui_plot::{Line, PlotPoints};
use super::Drawable;

impl Drawable<VaneData> for VaneData {
    fn draw(data: &Vec<VaneData>, ctx: &egui::Context) {
        if let Some(vane_data) = data.last() {
            egui::Window::new(format!("Vane{:02x}",vane_data.id))
                .vscroll(true)
                .show(ctx, |ui| {
                    ui.heading(format!("angle:\t{}", vane_data.angle));
                    ui.add_space(10.0);
                    ui.label(format!("timestamp:\t{}ms", vane_data.timestamp));
                    let plt=egui_plot::Plot::new("angle");
                    let theta=(-vane_data.angle as f64/180.0+0.5)*PI;
                    let circle_points:PlotPoints=(0..512)
                    .map(|i|{
                        let theta=PI*(i as f64)/512.0;
                        [
                            5.0*theta.cos(),
                            5.0*theta.sin()
                        ]
                    }).collect();
                    let circle_line=Line::new(circle_points)
                        .color(Color32::from_rgb(127,127,127))
                        .name("vane_circle");
                    let arrow_line=Line::new(
                        PlotPoints::new(
                            vec!(
                                [0.0,0.0],
                                [5.0*theta.cos(),5.0*theta.sin()]
                            )
                        )
                    ).color(Color32::from_rgb(255, 0, 0));
                    plt.show(ui, |plot_ui| {
                        plot_ui.line(circle_line);
                        plot_ui.line(arrow_line);
                    });
                });
        }
    }
}