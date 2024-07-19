use crate::parse::Data;
use byteorder::{BigEndian, ByteOrder};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct TachData{
    pub id: u8,
    pub timestamp: u32,
    pub cadence:f32,
    pub power:f32
}

impl Data for TachData{
    fn parse(data: &Vec<u8>) -> Self{
        TachData{
            id: data[0],
            timestamp: BigEndian::read_u32(&data[4..8]),
            cadence: BigEndian::read_f32(&data[12..16]),
            power: BigEndian::read_f32(&data[8..12]),
        }
    }
    fn get_size() -> usize {
        16
    }
}