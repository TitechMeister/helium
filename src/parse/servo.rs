use byteorder::{ByteOrder, BigEndian};
use crate::parse::Data;

#[allow(dead_code)]
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
    pub position_rudder:f32,
    pub position_elevator:f32,
    pub temperature_rudder:f32,
    pub temperature_elevator:f32,
}

impl Data for ServoData{
    fn parse(data: &Vec<u8>) -> Self{
        ServoData{
            id: data[0],
            status: data[1],
            timestamp: BigEndian::read_u32(&data[4..8]),
            rudder: BigEndian::read_f32(&data[8..12]),
            elevator: BigEndian::read_f32(&data[12..16]),
            voltage: BigEndian::read_f32(&data[16..20]),
            current_rudder: BigEndian::read_f32(&data[20..24]),
            current_elevator: BigEndian::read_f32(&data[24..28]),
            trim: BigEndian::read_f32(&data[28..32]),
            position_rudder: BigEndian::read_f32(&data[32..36]),
            position_elevator: BigEndian::read_f32(&data[36..40]),
            temperature_rudder: BigEndian::read_f32(&data[40..44]),
            temperature_elevator: BigEndian::read_f32(&data[44..48]),
        }
    }
    fn get_size() -> usize {
        49
    }
    fn get_buf_size() -> usize {
        0
    }
}