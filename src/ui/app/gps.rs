use std::f64::consts::PI;

use byteorder::{BigEndian, ByteOrder};
use eframe::egui::{self, ColorImage, Stroke};

pub struct Gps {
    zoom_level: u8,
    lat: f64,
    lon: f64,
    pos: Option<egui::Vec2>,
    goal: Option<(f64, f64)>,
    img: ColorImage,
}

impl Gps {
    pub fn new() -> Self {
        let img = image::open("assets/map/tile.png").unwrap().to_rgb8();
        let color_img =
            ColorImage::from_rgb([img.width() as usize, img.height() as usize], img.as_raw());
        Self {
            zoom_level: 12,
            lat: 35.297,
            lon: 136.178,
            pos: None,
            goal: None,
            img: color_img,
        }
    }
}

impl super::AppUI for Gps {
    fn update(&mut self, data: &mut crate::parse::Parser, ctx: &eframe::egui::Context) {

        // let platform_pos = (35.294230, 136.254344);
        // let takeshima_pos = (35.416626, 136.124324);
        // let okishima_pos = (35.250789, 136.063712);
        // let default_zoom_level = 12;
        let default_map_x = 3594;
        let default_map_y = 1615;

        egui::Window::new(format!("GPS"))
            .default_width(600.0)
            .default_height(400.0)
            .show(ctx, |ui| {
                // https://www.trail-note.net/tech/coordinate/

                let x = ((2.0_f64.powf(self.zoom_level as f64 + 7.0)) * (self.lon / 180.0 + 1.0))
                    as i64;
                let y = ((2.0_f64.powf(self.zoom_level as f64 + 7.0)) / PI
                    * (-self.lat.to_radians().sin().atanh()
                        + 85.05112878_f64.to_radians().sin().atanh()))
                    as i64;

                let map_x = x / 256;
                let map_y = y / 256;

                egui::CentralPanel::default().show_inside(ui, |ui| {
                    let size = ui.max_rect().width().min(ui.max_rect().height());
                    let plt = egui_plot::Plot::new("GPS").data_aspect(1.0);
                    let point: egui_plot::PlotPoints = data
                        .get_gps_data()
                        .iter()
                        .map(|gps_data| {
                            let x = ((2.0_f64.powf(self.zoom_level as f64 + 7.0))
                                * (gps_data.longitude / 180.0 + 1.0))
                                as f64;
                            let y = ((2.0_f64.powf(self.zoom_level as f64 + 7.0)) / PI
                                * (-gps_data.latitude.to_radians().sin().atanh()
                                    + 85.05112878_f64.to_radians().sin().atanh()))
                                as f64;
                            [x, y]
                        })
                        .collect();
                    let line = egui_plot::Line::new(point)
                        .color(egui::Color32::from_rgb(0, 64, 255))
                        .name("path");

                    let texture = ctx.load_texture("map", self.img.clone(), Default::default());
                    let image = egui_plot::PlotImage::new(
                        &texture,
                        egui_plot::PlotPoint::new(
                            (self.img.width() / 2) as f64,
                            -((self.img.height() / 2) as f64),
                        ),
                        (self.img.width() as f32, self.img.height() as f32),
                    );
                    plt.show(ui, |plot_ui| {
                        plot_ui.image(image);
                        plot_ui.line(line);
                        plot_ui.hline(
                            egui_plot::HLine::new(-((y-256*default_map_y) as f64))
                                .color(egui::Color32::from_rgb(255, 0, 0))
                                .name("current")
                                .width(3.0),
                        );
                        plot_ui.vline(
                            egui_plot::VLine::new((x-256*default_map_x) as f64)
                                .color(egui::Color32::from_rgb(255, 0, 0))
                                .name("current")
                                .width(3.0),
                        );
                    });
                    let (response, painter) =
                        ui.allocate_painter(ui.available_size_before_wrap(), egui::Sense::drag());

                    if let Some(pointer_pos) = response.interact_pointer_pos() {
                        let mouse_pos = (pointer_pos - ui.max_rect().min) / size * 256.0;
                        if mouse_pos.x >= 0.0
                            && mouse_pos.x <= 256.0
                            && mouse_pos.y >= 0.0
                            && mouse_pos.y <= 256.0
                        {
                            self.pos = Some(mouse_pos);
                        }
                    }

                    if let Some(pos) = self.pos {
                        painter.circle_stroke(
                            ui.max_rect().min + (pos.x * size / 256.0, pos.y * size / 256.0).into(),
                            5.0,
                            egui::Stroke::new(5.0, egui::Color32::RED),
                        );
                    }

                    if let Some((lon, lat)) = self.goal {
                        let goal_x = ((2.0_f64.powf(self.zoom_level as f64 + 7.0))
                            * (lon / 180.0 + 1.0)) as i64;
                        let goal_y = ((2.0_f64.powf(self.zoom_level as f64 + 7.0)) / PI
                            * (-lat.to_radians().sin().atanh()
                                + 85.05112878_f64.to_radians().sin().atanh()))
                            as i64;
                        let origin = ui.max_rect().min
                            + (
                                size / 256.0 * (x % 256) as f32,
                                size / 256.0 * (y % 256) as f32,
                            )
                                .into();
                        let goal = ui.max_rect().min
                            + (
                                size / 256.0 * (goal_x - map_x * 256) as f32,
                                size / 256.0 * (goal_y - map_y * 256) as f32,
                            )
                                .into();
                        painter.line_segment([origin, goal], Stroke::new(1.0, egui::Color32::RED));
                    }
                });

                egui::SidePanel::right("GPS_r_panel")
                    .resizable(true)
                    .show_inside(ui, |ui| {
                        if let Some(gps_data) = data.get_gps_data().last() {
                            ui.heading(format!("lon:\t{}", gps_data.longitude));
                            ui.heading(format!("lat:\t{}", gps_data.latitude));
                            ui.add_space(10.0);
                            ui.label(format!("timestamp:\t{}ms", gps_data.timestamp));
                        }

                        if let Some(pos) = self.pos {
                            // https://www.trail-note.net/tech/coordinate/

                            let pos_lon = 180.0
                                * ((pos.x as f64 + 256.0 * map_x as f64)
                                    / 2.0_f64.powf(self.zoom_level as f64 + 7.0)
                                    - 1.0);
                            let pos_lat = 180.0 / PI
                                * ((-PI * (pos.y as f64 + 256.0 * map_y as f64)
                                    / 2.0_f64.powf(self.zoom_level as f64 + 7.0)
                                    + 85.05112878_f64.to_radians().sin().atanh())
                                .tanh())
                                .asin();

                            // ui.label(format!("mouse pos:\t{:1.3},{:1.3}", pos.x, pos.y)); // pixel coordinate
                            ui.label(format!("mouse pos:\t{:1.3},{:1.3}", pos_lon, pos_lat)); // GPS coordinate
                            if ui.button("set goal").clicked() {
                                self.goal = Some((pos_lon, pos_lat));
                                let mut bytes = [0u8; 20];
                                bytes[0] = 0x01; // message id
                                BigEndian::write_f64(&mut bytes[4..12], pos_lon);
                                BigEndian::write_f64(&mut bytes[12..20], pos_lat);
                                data.write(&bytes.to_vec());
                            }
                        }
                        if let Some((lon, lat)) = self.goal {
                            ui.label(format!("goal:\t{:1.3},{:1.3}", lon, lat));
                            ui.add_space(20.0);
                            ui.heading(format!(
                                "Distance:\t{:1.3}m",
                                ((self.lat - lat) / 0.000008983148616)
                                    .hypot((self.lon - lon) / 0.000010966382364)
                            ));
                        }
                    });
            });
    }
}
