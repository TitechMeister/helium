use std::f64::consts::{PI, TAU};

use byteorder::{BigEndian, ByteOrder};
use eframe::egui::{self, ColorImage};

pub struct Gps {
    /// Latitude
    lat: f64,
    /// Longitude
    lon: f64,
    /// Zoom level
    zoom_level: u8,
    /// Map x coordinate
    map_x: i64,
    /// Map y coordinate
    map_y: i64,
    /// Selected position (pixel coordinate)
    pos: Option<(f64, f64)>,
    /// Goal position (GPS coordinate)
    goal: Option<(f64, f64)>,
    /// Map image
    img: Option<ColorImage>,
    is_tracking: bool,
    area_size: f64,
}

impl Gps {
    pub fn new() -> Self {
        // let map = include_bytes!("../../../assets/map/14-14377-6461.png");
        let map = include_bytes!("../../assets/map/12-3594-1615.png");
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

    /// タイルマップの取得
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
    /// GPS座標をピクセル座標に変換
    fn gps2pixel(&self, lat: f64, lon: f64) -> (f64, f64) {
        // https://www.trail-note.net/tech/coordinate/
        let x = (2.0_f64.powf(self.zoom_level as f64 + 7.0)) * (lon / 180.0 + 1.0);
        let y = (2.0_f64.powf(self.zoom_level as f64 + 7.0)) / PI
            * (-lat.to_radians().sin().atanh() + 85.05112878_f64.to_radians().sin().atanh());
        (x-(self.map_x*256) as f64, -(y-(self.map_y*256) as f64))
    }
    /// ピクセル座標をGPS座標に変換
    fn pixel2gps(&self, x: f64, y: f64) -> (f64, f64) {
        // https://www.trail-note.net/tech/coordinate/
        let lon = 180.0
            * ((x + 256.0 * self.map_x as f64) / 2.0_f64.powf(self.zoom_level as f64 + 7.0)
                - 1.0);
        let lat = 180.0 / PI
            * ((-PI * (-y + 256.0 * self.map_y as f64)
                / 2.0_f64.powf(self.zoom_level as f64 + 7.0)
                + 85.05112878_f64.to_radians().sin().atanh())
            .tanh())
            .asin();
        (lat, lon)
    }
}

impl super::AppUI for Gps {
    fn update(&mut self, data: &mut crate::parse::Parser, ctx: &eframe::egui::Context) {
        // let platform_pos = (35.294230, 136.254344);
        let chikubushima_pos = (35.416626, 136.124324);
        let okishima_pos = (35.258477, 136.061343);
        let mid_pos = (35.340891, 136.064750);
        let pylon1km_pos = (35.296822, 136.243811);

        egui::Window::new(format!("GPS")).show(ctx, |ui| {
            let (x, y) = self.gps2pixel(self.lat, self.lon);

            // Control Panel

            egui::SidePanel::right("GPS_r_panel")
                .resizable(false)
                .default_width(400.0)
                .show_inside(ui, |ui| {
                    if let Some((gps_data, _)) = data.get_gps_data().last() {
                        self.lat = gps_data.latitude;
                        self.lon = gps_data.longitude;
                    }

                    if let Some(pos) = self.pos {
                        // https://www.trail-note.net/tech/coordinate/

                        let (pos_lat, pos_lon) = self.pixel2gps(pos.0, pos.1);

                        // 目的地の設定
                        if ui.button("Set Goal").clicked() {
                            self.goal = Some((pos_lat, pos_lon));
                            self.pos = None;
                            let mut bytes = [0u8; 32];
                            bytes[0] = 0x01; // message id
                            BigEndian::write_f64(&mut bytes[8..16], pos_lon);
                            BigEndian::write_f64(&mut bytes[16..24], pos_lat);
                            data.write(&bytes.to_vec());
                        }
                    }
                    if let Some((lat, lon)) = self.goal {
                        ui.horizontal(|ui|{
                            ui.heading(format!(
                                "Distance:\t{:1.3}m",
                                ((self.lat - lat) / 0.000008983148616)
                                    .hypot((self.lon - lon) / 0.000010966382364)
                            ));
                            ui.add_space(10.0);
                            if ui.button("再送信").clicked() {
                                let mut bytes = [0u8; 32];
                                bytes[0] = 0x01; // message id
                                BigEndian::write_f64(&mut bytes[8..16], lon);
                                BigEndian::write_f64(&mut bytes[16..24], lat);
                                data.write(&bytes.to_vec());
                            }
                        });
                    }

                    ui.add_space(10.0);

                    ui.heading(egui::RichText::new("Goal Position").color(egui::Color32::from_rgb(255, 127, 0)));


                    if ui.button("竹生島").clicked() {
                        self.goal = Some(chikubushima_pos);
                    }

                    if ui.button("沖島").clicked() {
                        self.goal = Some(okishima_pos);
                    }

                    if ui.button("パイロン(1km)").clicked() {
                        self.goal = Some(pylon1km_pos);
                    }

                    ui.add_space(20.0);

                    // Tracking Modeの設定
                    ui.checkbox(&mut self.is_tracking, "Tracking");

                    ui.add_space(10.0);

                    // Tracking ModeのArea Sizeの設定
                    if self.is_tracking {
                        ui.label("Area Size");
                        ui.add(
                            egui::Slider::new(&mut self.area_size, 10.0..=1000.0)
                                .orientation(egui::SliderOrientation::Vertical)
                                .show_value(false),
                        );
                    }
                });

            // Plot Area
            egui::CentralPanel::default().show_inside(ui, |ui| {
                let plt = egui_plot::Plot::new("GPS").data_aspect(1.0);

                // 軌跡をプロット
                let point: egui_plot::PlotPoints = data
                    .get_gps_data()
                    .iter()
                    .map(|(gps_data, _)| {
                        let (x, y) = self.gps2pixel(gps_data.latitude, gps_data.longitude);
                        [x as f64, y as f64]
                    })
                    .collect();
                let line = egui_plot::Line::new(point)
                    .color(egui::Color32::from_rgb(0xff, 0x70, 0x00))
                    .name("path");

                // タイルマップを表示
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
                    // 失敗した場合はデフォルトのタイルマップを表示
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
                        egui_plot::HLine::new(y as f64)
                            .color(egui::Color32::from_rgb(255, 0, 255))
                            .name("current")
                            .width(0.5),
                    );
                    plot_ui.vline(
                        egui_plot::VLine::new(x as f64)
                            .color(egui::Color32::from_rgb(255, 0, 255))
                            .name("current")
                            .width(0.5),
                    );

                    // クリックした位置をプロット
                    if let Some(pos) = self.pos {
                        plot_ui.add(
                            egui_plot::Points::new(vec![[pos.0, pos.1]])
                                .color(egui::Color32::from_rgb(255, 0, 0))
                                .name("Goal (tantative)")
                                .shape(egui_plot::MarkerShape::Diamond)
                                .radius(5.0),
                        );
                    }

                    // 旋回ポイントをプロット
                    let radius = 20.0;

                    // プラホ
                    let (pylon1km_x, pylon1km_y) = self.gps2pixel(pylon1km_pos.0, pylon1km_pos.1);
                    plot_ui.line(
                        egui_plot::Line::new(egui_plot::PlotPoints::new(
                            (0..=512)
                                .map(|i| {
                                    let theta = TAU / 512.0 * i as f64;
                                    [
                                        pylon1km_x as f64
                                            + radius * theta.cos(),
                                        pylon1km_y as f64
                                            + radius * theta.sin(),
                                    ]
                                })
                                .collect(),
                        ))
                        .color(egui::Color32::from_rgb(0xFF, 0, 0)),
                    );

                    // 竹生島
                    let (chikubushima_x, chikubushima_y) =
                        self.gps2pixel(chikubushima_pos.0, chikubushima_pos.1);
                    plot_ui.line(
                        egui_plot::Line::new(egui_plot::PlotPoints::new(
                            (0..=512)
                                .map(|i| {
                                    let theta = TAU / 512.0 * i as f64;
                                    [
                                        chikubushima_x as f64
                                            + radius * theta.cos(),
                                        chikubushima_y as f64
                                            + radius * theta.sin(),
                                    ]
                                })
                                .collect(),
                        ))
                        .color(egui::Color32::from_rgb(0xFF, 0, 0)),
                    );

                    // 沖島
                    let (okishima_x, okishima_y) = self.gps2pixel(okishima_pos.0, okishima_pos.1);
                    plot_ui.line(
                        egui_plot::Line::new(egui_plot::PlotPoints::new(
                            (0..=512)
                                .map(|i| {
                                    let theta = TAU / 512.0 * i as f64;
                                    [
                                        okishima_x as f64
                                            + radius * theta.cos(),
                                        okishima_y as f64
                                            + radius * theta.sin(),
                                    ]
                                })
                                .collect(),
                        ))
                        .color(egui::Color32::from_rgb(0xFF, 0, 0)),
                    );

                    // 二等分線の表示
                    let (mid_x, mid_y) = self.gps2pixel(mid_pos.0, mid_pos.1);
                    plot_ui.line(
                        egui_plot::Line::new(egui_plot::PlotPoints::new(vec![
                            [
                                mid_x as f64,
                                mid_y as f64,
                            ],
                            [
                                pylon1km_x as f64,
                                pylon1km_y as f64,
                            ],
                        ]))
                        .width(1.0),
                    );

                    // ゴールをプロット
                    if let Some((lat, lon)) = self.goal {
                        let (gx, gy) = self.gps2pixel(lat, lon);

                        plot_ui.line(
                            egui_plot::Line::new(egui_plot::PlotPoints::new(vec![
                                [
                                    x as f64,
                                    y as f64,
                                ],
                                [
                                    gx as f64,
                                    gy as f64,
                                ],
                            ]))
                            .color(egui::Color32::RED)
                            .width(3.0),
                        );

                        plot_ui.add(
                            egui_plot::Points::new(vec![[
                                gx as f64,
                                gy as f64,
                            ]])
                            .color(egui::Color32::from_rgb(255, 0, 0))
                            .name("Goal")
                            .shape(egui_plot::MarkerShape::Diamond)
                            .radius(5.0),
                        );
                    }

                    plot_ui.line(egui_plot::Line::new(egui_plot::PlotPoints::new(
                        (0..=10)
                            .map(|i| {
                                let x = (i as f64 - 5.0) * 256.0 + x - (self.map_x * 256) as f64;
                                [x, y as f64]
                            })
                            .collect(),
                    )));

                    // Tracking Mode :
                    //  - 現在位置を中心に表示
                    //  - エリアサイズ(area_size)を変更可能

                    if self.is_tracking {
                        plot_ui.set_plot_bounds(egui_plot::PlotBounds::from_min_max(
                            [
                                x as f64 - self.area_size,
                                y as f64 - self.area_size,
                            ],
                            [
                                x as f64 + self.area_size,
                                y as f64 + self.area_size,
                            ],
                        ));
                    }
                    plot_ui.pointer_coordinate()
                });

                // クリックした位置を取得

                if let Some(_) = response.interact_pointer_pos() {
                    if let Some(pos) = pointer_coordinate {
                        self.pos = Some((pos.x, pos.y));
                    }
                }
            });
        });
    }
}
