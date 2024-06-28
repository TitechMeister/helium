use crate::parse::ServoData;
use eframe::egui;
use super::Drawable;

impl Drawable<ServoData> for ServoData {
    fn draw(data: &Vec<ServoData>, ctx: &egui::Context) {
        if let Some(servo_data) = data.last() {
            egui::Window::new("Servo").vscroll(true).show(ctx, |ui| {
                ui.add(
                    eframe::egui::widgets::ProgressBar::new(servo_data.rudder / 40.0 + 0.5)
                        .text(format!("rudder:\t{:2.2}deg", servo_data.rudder)),
                );
                ui.add(
                    eframe::egui::widgets::ProgressBar::new(servo_data.elevator / 40.0 + 0.5)
                        .text(format!("elevator:\t{:2.2}deg", servo_data.elevator)),
                );
                ui.add(
                    eframe::egui::widgets::ProgressBar::new(servo_data.trim / 10.0 + 0.5)
                        .text(format!("trim:\t{:2.2}deg", servo_data.trim)),
                );
                ui.add_space(10.0);
                if servo_data.voltage != 0.0 {
                    ui.add(
                        eframe::egui::widgets::ProgressBar::new(servo_data.voltage / 140.0)
                            .text(format!("voltage:\t{:2.2}V", servo_data.voltage / 10.0)),
                    );
                } else {
                    ui.add(
                        eframe::egui::widgets::ProgressBar::new(0.0)
                            .text(format!("voltage:\t{:2.2}V", servo_data.voltage / 10.0)),
                    );
                }
                ui.add(
                    eframe::egui::widgets::ProgressBar::new(servo_data.current_rudder / 2000.0 + 0.5)
                        .text(format!("i_ruuder:\t{:4.2}mA", servo_data.current_rudder)),
                );
                ui.add(
                    eframe::egui::widgets::ProgressBar::new(servo_data.current_elevator / 2000.0 + 0.5)
                        .text(format!("i_elevator:\t{:4.2}mA", servo_data.current_elevator)),
                );
                ui.heading(format!("status:\t{}", servo_data.status));
                ui.add_space(10.0);
                ui.label(format!("time:\t{}", servo_data.timestamp));
            });
        }
    }
}