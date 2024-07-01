use std::f64::consts::PI;

use eframe::egui;

pub struct Gps {
    zoom_level: u8,
    lat: f64,
    lon: f64,
    pos: Option<egui::Vec2>,
}

impl Gps {
    pub fn new() -> Self {
        Self {
            zoom_level: 10,
            lat: 35.3_f64,
            lon: 136.1899_f64,
            pos: None,
        }
    }
}

impl super::AppUI for Gps {
    fn update(&mut self, data: &crate::parse::Parser, ctx: &eframe::egui::Context) {
        egui::Window::new(format!("GPS")).show(ctx, |ui| {

             // https://www.trail-note.net/tech/coordinate/

            let x =
                ((2.0_f64.powf(self.zoom_level as f64 + 7.0)) * (self.lon / 180.0 + 1.0)) as i64;
            let y = ((2.0_f64.powf(self.zoom_level as f64 + 7.0)) / PI
                * (-self.lat.to_radians().sin().atanh()
                    + 85.05112878_f64.to_radians().sin().atanh())) as i64;

            let map_x = x / 256;
            let map_y = y / 256;

            egui::CentralPanel::default().show_inside(ui, |ui| {
                let size = ui.max_rect().width().min(ui.max_rect().height());
                egui::Image::from_uri(format!(
                    "file://assets/map/{}/{}_{}.png",
                    self.zoom_level, map_y, map_x
                ))
                .fit_to_exact_size((size, size).into())
                .paint_at(
                    ui,
                    egui::Rect::from_min_size(ui.min_rect().min, egui::vec2(size, size)),
                );
                let (response, painter) =
                    ui.allocate_painter(ui.available_size_before_wrap(), egui::Sense::drag());

                if let Some(pointer_pos) = response.interact_pointer_pos() {
                    self.pos = Some((pointer_pos - ui.max_rect().min) / size * 256.0);
                }

                if let Some(pos) = self.pos {
                    painter.circle_stroke(
                        ui.max_rect().min + (pos.x * size / 256.0, pos.y * size / 256.0).into(),
                        5.0,
                        egui::Stroke::new(5.0, egui::Color32::RED),
                    );
                }

                painter.circle_filled(
                    ui.max_rect().min
                        + (
                            size / 256.0 * (x % 256) as f32,
                            size / 256.0 * (y % 256) as f32,
                        )
                            .into(),
                    5.0,
                    egui::Color32::RED,
                );
            });

            egui::SidePanel::right("GPS_r_panel")
                .resizable(true)
                .default_width(150.0)
                .show_inside(ui, |ui| {
                    if let Some(gps_data) = data.get_gps_data().last() {
                        ui.heading(format!("lon:\t{}", gps_data.longitude));
                        ui.heading(format!("lat:\t{}", gps_data.latitude));
                        ui.add_space(10.0);
                        ui.label(format!("timestamp:\t{}ms", gps_data.timestamp));
                    }
                    ui.add(egui::Slider::new(&mut self.zoom_level, 10..=13).text("zoom level"));
                    if let Some(pos) = self.pos {

                        // https://www.trail-note.net/tech/coordinate/
                        
                        let pos_lon = 180.0
                            * ((pos.x as f64 + 256.0 * map_x as f64 ) / 2.0_f64.powf(self.zoom_level as f64 + 7.0) - 1.0);
                        let pos_lat = 180.0 / PI
                            * ((- PI * (pos.y as f64 + 256.0 * map_y as f64) / 2.0_f64.powf(self.zoom_level as f64 + 7.0)+85.05112878_f64.to_radians().sin().atanh()).tanh())
                            .asin();

                        ui.label(format!("mouse pos:\t{:1.3},{:1.3}", pos.x, pos.y));
                        ui.label(format!("mouse pos:\t{:1.3},{:1.3}", pos_lon, pos_lat));
                    }
                    ui.label(format!(
                        "file://assets/map/{}/{}_{}.png",
                        self.zoom_level, map_y, map_x
                    ));
                });
        });
    }
}
