pub mod altitude;
// pub mod flight_menu;
pub mod gps;
pub mod imu;
pub mod pitot;
pub mod servo;
pub mod tachometer;
pub mod vane;

pub trait AppUI{
    fn update(&mut self,data:&mut crate::parse::Parser,ctx:&eframe::egui::Context);
}