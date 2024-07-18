use crate::parse::Data;
use eframe::egui;

mod alt_data;
mod barometer;
mod gps;
mod vane;
mod imu_data;
mod pitot_data;
mod servo_data;
mod tachometer;

pub trait Drawable<T>
where
    T: Data + Copy + Clone,
{
    fn draw(data: &Vec<T>, ctx: &egui::Context);
}