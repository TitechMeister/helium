use byteorder::{ByteOrder, BigEndian};
use crate::parse::Data;

#[derive(Debug)]
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
}