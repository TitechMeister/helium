use crate::parse::Data;
use byteorder::{BigEndian, ByteOrder};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct UltraSonicData{
    pub id: u8,
    pub timestamp: u32,
    pub altitude:f32
}

impl Data for UltraSonicData{
    fn parse(data: &Vec<u8>) -> Self{
        UltraSonicData{
            id: data[0],
            timestamp: BigEndian::read_u32(&data[4..8]),
            altitude: BigEndian::read_f32(&data[8..12]),
        }
    }
    fn get_size() -> usize {
        12
    }
    fn get_buf_size() -> usize {
        0
    }
}