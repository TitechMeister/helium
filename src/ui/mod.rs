pub mod altitude;
// pub mod flight_menu;
pub mod gps;
pub mod imu;
pub mod pitot_data;
pub mod servo_data;
pub mod tachometer;
pub mod vane;

pub trait AppUI{
    fn update(&mut self,data:&mut crate::parse::Parser,ctx:&eframe::egui::Context);
}