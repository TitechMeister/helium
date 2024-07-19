use eframe::egui::epaint::{Color32, Stroke};
use eframe::egui::{self, Sense};
use super::AppUI;

pub struct ServoUI {}

impl ServoUI {
    pub fn new() -> Self {
        Self {}
    }
}

impl AppUI for ServoUI {
    fn update(&mut self, data: &mut crate::parse::Parser, ctx: &eframe::egui::Context) {
        if let Some((servo_data,_)) = data.get_servo_data().last() {
            egui::Window::new("Servo").vscroll(true).show(ctx, |ui| {
                egui::SidePanel::right("Servo_l_panel").show_inside(ui, |ui| {
                    let size = ui.available_size();
                    let (response, painter) = ui.allocate_painter(size, Sense::hover());
                    let rect = response.rect;
                    let mut c = rect.center();
                    c.x = rect.min.x + rect.width() / 2.0;
                    let r = (rect.width() / 2.0-1.0)/15.0;
                    let stroke_frame = Stroke::new(1.0, Color32::DARK_GRAY);
                    painter.circle_stroke(c, r*15.0, stroke_frame);
                    painter.circle_stroke(c, r*10.0, stroke_frame);
                    painter.circle_stroke(c, r*5.0, stroke_frame);
                    painter.circle_stroke(c, r*2.0, stroke_frame);
                    painter.circle_stroke(c, r*1.0, stroke_frame);
                    painter.line_segment([
                        rect.center(),
                        rect.center() + egui::vec2(servo_data.rudder*r, servo_data.elevator*r),
                    ], stroke_frame);
                    painter.circle_filled(
                        rect.center() + egui::vec2(servo_data.rudder*r, servo_data.elevator*r),
                        r,
                        Color32::RED);
                });

                ui.add(
                    eframe::egui::widgets::ProgressBar::new(servo_data.rudder / 40.0 + 0.5)
                        .text(format!("rudder:\t{:2.2}deg", servo_data.rudder)),
                );
                ui.add(
                    eframe::egui::widgets::ProgressBar::new(servo_data.elevator / 40.0 + 0.5)
                        .text(format!("elevator:\t{:2.2}deg", servo_data.elevator)),
                );

                ui.add(
                    eframe::egui::widgets::ProgressBar::new(servo_data.trim / 15.0 + 0.5)
                        .text(format!("trim:\t{:2.2}deg", servo_data.trim)),
                );
                ui.add_space(15.0);
                if servo_data.voltage != 0.0 {
                    ui.add(
                        eframe::egui::widgets::ProgressBar::new(servo_data.voltage / 140.0)
                            .text(format!("voltage:\t{:2.2}V", servo_data.voltage / 15.0)),
                    );
                } else {
                    ui.add(
                        eframe::egui::widgets::ProgressBar::new(0.0)
                            .text(format!("voltage:\t{:2.2}V", servo_data.voltage / 15.0)),
                    );
                }
                ui.add(
                    eframe::egui::widgets::ProgressBar::new(
                        servo_data.current_rudder / 2000.0 + 0.5,
                    )
                    .text(format!("i_ruuder:\t{:4.2}mA", servo_data.current_rudder)),
                );
                ui.add(
                    eframe::egui::widgets::ProgressBar::new(
                        servo_data.current_elevator / 2000.0 + 0.5,
                    )
                    .text(format!(
                        "i_elevator:\t{:4.2}mA",
                        servo_data.current_elevator
                    )),
                );
                ui.heading(format!("status:\t{}", servo_data.status));
                ui.add_space(15.0);
                ui.label(format!("time:\t{}", servo_data.timestamp));
            });
        }
    }
}
