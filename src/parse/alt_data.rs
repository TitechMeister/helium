use crate::parse::Data;
use byteorder::{BigEndian, ByteOrder};

#[derive(Debug, Clone, Copy)]
pub struct AltData{
    pub id: u8,
    pub timestamp: u32,
    pub altitude:f32
}

impl Data for AltData{
    fn parse(data: &Vec<u8>) -> Self{
        AltData{
            id: data[0],
            timestamp: BigEndian::read_u32(&data[4..8]),
            altitude: BigEndian::read_f32(&data[8..12]),
        }
    }
    fn draw(&self,ui:&mut eframe::egui::Ui) {
        ui.label(format!("altitude:\t{}",self.altitude));
        ui.add_space(10.0);
        ui.label(format!("timestamp:\t{}ms",self.timestamp));
    }
}