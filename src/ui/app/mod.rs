pub mod imu;

pub trait AppUI{
    fn update(&mut self,data:&crate::parse::Parser,ctx:&eframe::egui::Context);
}