use crate::parse::Data;
use byteorder::{BigEndian, ByteOrder};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct PitotData{
    pub id: u8,
    pub timestamp: u32,
    pub pressure:f32,
    pub temperature:f32,
    pub velocity:f32
}

impl Data for PitotData{
    fn parse(data: &Vec<u8>) -> Self{
        PitotData{
            id: data[0],
            timestamp: BigEndian::read_u32(&data[4..8]),
            pressure: BigEndian::read_f32(&data[8..12]),
            temperature:BigEndian::read_f32(&data[12..16]),
            velocity:BigEndian::read_f32(&data[16..20]),
        }
    }
    fn get_size() -> usize {
        20
    }
}