use std::f64::consts::PI;

use byteorder::{BigEndian, ByteOrder};
use eframe::egui::{self, ColorImage};

pub struct Gps {
    zoom_level: u8,
    lat: f64,
    lon: f64,
    map_x: i64,
    map_y: i64,
    pos: Option<(f64, f64)>,  // (x, y)
    goal: Option<(f64, f64)>, // (lat, lon)
    img: Option<ColorImage>,
    is_tracking: bool,
    area_size: f64,
}

impl Gps {
    pub fn new() -> Self {
        // let map = include_bytes!("../../../assets/map/14-14377-6461.png");
        let map = include_bytes!("../../../assets/map/12-3594-1615.png");
        let img = image::load_from_memory(map).unwrap();
        let color_img = ColorImage::from_rgb(
            [img.width() as usize, img.height() as usize],
            img.to_rgb8().as_raw(),
        );
        // Self {
        //     zoom_level: 14,
        //     lat: 35.294230,
        //     lon: 136.254344,
        //     map_x: 14377,
        //     map_y: 6461,
        //     pos: None,
        //     goal: None,
        //     img: Some(color_img),
        //     is_tracking: false,
        //     area_size: 500.0,
        // }
        Self {
            zoom_level: 12,
            lat: 35.294230,
            lon: 136.254344,
            map_x: 3594,
            map_y: 1615,
            pos: None,
            goal: None,
            img: Some(color_img),
            is_tracking: false,
            area_size: 500.0,
        }
    }
    fn open_img(&mut self, zoom: u8, map_x: i64, map_y: i64) {
        if let Ok(img) = image::open(format!("assets/map/{}-{}-{}.png", zoom, map_x, map_y)) {
            let color_img = ColorImage::from_rgb(
                [img.width() as usize, img.height() as usize],
                img.to_rgb8().as_raw(),
            );
            self.zoom_level = zoom;
            self.map_x = map_x;
            self.map_y = map_y;
            self.img = Some(color_img);
        }
    }
}

impl super::AppUI for Gps {
    fn update(&mut self, data: &mut crate::parse::Parser, ctx: &eframe::egui::Context) {
        // let platform_pos = (35.294230, 136.254344);
        // let takeshima_pos = (35.416626, 136.124324);
        // let okishima_pos = (35.250789, 136.063712);

        egui::Window::new(format!("GPS")).show(ctx, |ui| {
            // https://www.trail-note.net/tech/coordinate/

            let x =
                ((2.0_f64.powf(self.zoom_level as f64 + 7.0)) * (self.lon / 180.0 + 1.0)) as i64;
            let y = ((2.0_f64.powf(self.zoom_level as f64 + 7.0)) / PI
                * (-self.lat.to_radians().sin().atanh()
                    + 85.05112878_f64.to_radians().sin().atanh())) as i64;

            // Control Panel

            egui::SidePanel::right("GPS_r_panel")
                .resizable(false)
                .default_width(400.0)
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
                            * ((pos.0 as f64 + 256.0 * self.map_x as f64)
                                / 2.0_f64.powf(self.zoom_level as f64 + 7.0)
                                - 1.0);
                        let pos_lat = 180.0 / PI
                            * ((-PI * (-pos.1 as f64 + 256.0 * self.map_y as f64)
                                / 2.0_f64.powf(self.zoom_level as f64 + 7.0)
                                + 85.05112878_f64.to_radians().sin().atanh())
                            .tanh())
                            .asin();

                        if ui.button("Set Goal").clicked() {
                            self.goal = Some((pos_lat, pos_lon));
                            self.pos = None;
                            let mut bytes = [0u8; 20];
                            bytes[0] = 0x01; // message id
                            BigEndian::write_f64(&mut bytes[4..12], pos_lon);
                            BigEndian::write_f64(&mut bytes[12..20], pos_lat);
                            data.write(&bytes.to_vec());
                        }
                    }
                    if let Some((lat, lon)) = self.goal {
                        ui.label(format!("Goal:\t{:1.3},{:1.3}", lat, lon));
                        ui.add_space(20.0);
                        ui.heading(format!(
                            "Distance:\t{:1.3}m",
                            ((self.lat - lat) / 0.000008983148616)
                                .hypot((self.lon - lon) / 0.000010966382364)
                        ));
                    }

                    ui.add_space(20.0);

                    ui.checkbox(&mut self.is_tracking, "Tracking");

                    ui.add_space(10.0);

                    if self.is_tracking {
                        ui.label("Area Size");
                        ui.add(
                            egui::Slider::new(&mut self.area_size, 10.0..=1000.0)
                                .orientation(egui::SliderOrientation::Vertical)
                                .show_value(false)
                        );
                    }
                });

            // Plot Area

            egui::CentralPanel::default().show_inside(ui, |ui| {
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
                    .color(egui::Color32::from_rgb(0, 0, 0))
                    .name("path");

                let mut image: Option<egui_plot::PlotImage> = None;

                if let Some(img) = &self.img {
                    let texture = ctx.load_texture("map", img.clone(), Default::default());
                    image = Some(egui_plot::PlotImage::new(
                        &texture,
                        egui_plot::PlotPoint::new(
                            (img.width() / 2) as f64,
                            -((img.height() / 2) as f64),
                        ),
                        (img.width() as f32, img.height() as f32),
                    ));
                } else {
                    self.open_img(12, 3594, 1615);
                }

                let egui_plot::PlotResponse {
                    response,
                    inner: pointer_coordinate,
                    ..
                } = plt.show(ui, |plot_ui| {
                    if let Some(img) = image {
                        plot_ui.image(img);
                    }
                    plot_ui.line(line);
                    plot_ui.hline(
                        egui_plot::HLine::new(-((y - 256 * self.map_y) as f64))
                            .color(egui::Color32::from_rgb(255, 0, 255))
                            .name("current")
                            .width(0.5),
                    );
                    plot_ui.vline(
                        egui_plot::VLine::new((x - 256 * self.map_x) as f64)
                            .color(egui::Color32::from_rgb(255, 0, 255))
                            .name("current")
                            .width(0.5),
                    );
                    if let Some(pos) = self.pos {
                        plot_ui.add(
                            egui_plot::Points::new(vec![[pos.0, pos.1]])
                                .color(egui::Color32::from_rgb(255, 0, 0))
                                .name("Goal (tantative)")
                                .shape(egui_plot::MarkerShape::Diamond)
                                .radius(5.0)
                        );
                    }
                    if let Some((lat, lon)) = self.goal {
                        let gx = ((2.0_f64.powf(self.zoom_level as f64 + 7.0))
                            * (lon / 180.0 + 1.0)) as i64;
                        let gy = ((2.0_f64.powf(self.zoom_level as f64 + 7.0)) / PI
                            * (-lat.to_radians().sin().atanh()
                                + 85.05112878_f64.to_radians().sin().atanh()))
                            as i64;

                        plot_ui.line(
                            egui_plot::Line::new(egui_plot::PlotPoints::new(vec![
                                [
                                    ((x - self.map_x * 256) as f64),
                                    -((y - 256 * self.map_y) as f64),
                                ],
                                [
                                    ((gx - self.map_x * 256) as f64),
                                    -((gy - self.map_y * 256) as f64),
                                ],
                            ]))
                            .color(egui::Color32::RED)
                            .width(3.0),
                        );

                        plot_ui.add(
                            egui_plot::Points::new(vec![[(gx - self.map_x * 256) as f64, -(gy - self.map_y * 256) as f64]])
                                .color(egui::Color32::from_rgb(255, 0, 0))
                                .name("Goal")
                                .shape(egui_plot::MarkerShape::Diamond)
                                .radius(5.0)
                        );
                    }

                    if self.is_tracking {
                        plot_ui.set_plot_bounds(egui_plot::PlotBounds::from_min_max(
                            [
                                (x - self.map_x * 256) as f64 - self.area_size,
                                -(y - self.map_y * 256) as f64 - self.area_size,
                            ],
                            [
                                (x - self.map_x * 256) as f64 + self.area_size,
                                -(y - self.map_y * 256) as f64 + self.area_size,
                            ],
                        ));
                    }
                    plot_ui.pointer_coordinate()
                });

                if let Some(_) = response.interact_pointer_pos() {
                    if let Some(pos) = pointer_coordinate {
                        self.pos = Some((pos.x, pos.y));
                    }
                }
            });
        });
    }
}
