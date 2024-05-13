use byteorder::{ByteOrder, BigEndian};
use crate::parse::Data;

#[derive(Debug, Clone, Copy)]
pub struct ServoData{
    pub id: u8,
    pub timestamp: u32,
    pub rudder:f32,
    pub elevator:f32,
    pub voltage:f32,
    pub current_rudder:f32,
    pub current_elevator:f32,
    pub trim:f32,
    pub status:u8,
}

impl Data for ServoData{
    fn parse(data: &Vec<u8>) -> Self{
        ServoData{
            id: data[0],
            timestamp: BigEndian::read_u32(&data[4..8]),
            rudder: BigEndian::read_f32(&data[8..13]),
            elevator: BigEndian::read_f32(&data[12..16]),
            voltage: BigEndian::read_f32(&data[16..20]),
            current_rudder: BigEndian::read_f32(&data[20..24]),
            current_elevator: BigEndian::read_f32(&data[24..28]),
            trim: BigEndian::read_f32(&data[28..32]),
            status: data[32],
        }
    }

    fn draw(&self,ui:&mut eframe::egui::Ui){
        ui.add(eframe::egui::widgets::ProgressBar::new(self.rudder/40.0+0.5).text(format!("rudder:\t{:2.2}deg",self.rudder)));
        ui.add(eframe::egui::widgets::ProgressBar::new(self.elevator/40.0+0.5).text(format!("elevator:\t{:2.2}deg",self.elevator)));
        ui.add(eframe::egui::widgets::ProgressBar::new(self.trim/5.0+0.5).text(format!("trim:\t{:2.2}deg",self.trim)));
        ui.add_space(10.0);
        if self.voltage != 0.0{
            ui.add(eframe::egui::widgets::ProgressBar::new(self.voltage/140.0).text(format!("voltage:\t{:2.2}V",self.voltage/10.0)));
        }else{
            ui.add(eframe::egui::widgets::ProgressBar::new(0.0).text(format!("voltage:\t{:2.2}V",self.voltage/10.0)));
        }
        ui.add(eframe::egui::widgets::ProgressBar::new(self.current_rudder/2000.0+0.5).text(format!("i_ruuder:\t{:4.2}mA",self.current_rudder)));
        ui.add(eframe::egui::widgets::ProgressBar::new(self.current_elevator/2000.0+0.5).text(format!("i_elevator:\t{:4.2}mA",self.current_elevator)));
        ui.label(format!("status:\t{}",self.status));
        ui.add_space(10.0);
        ui.label(format!("timestamp:\t{}ms",self.timestamp));
    }
}