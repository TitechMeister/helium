pub mod imu;
pub mod flight_menu;
pub mod gps;

pub trait AppUI{
    fn update(&mut self,data:&mut crate::parse::Parser,ctx:&eframe::egui::Context);
}