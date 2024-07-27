use crate::parse::Data;
use byteorder::{BigEndian, ByteOrder};

#[derive(Debug, Clone, Copy)]
pub struct VaneData{
    pub id: u8,
    pub timestamp: u32,
    pub angle: f32,
}

impl Data for VaneData{
    fn parse(data: &Vec<u8>) -> Self{
        VaneData{
            id: data[0],
            timestamp: BigEndian::read_u32(&data[4..8]),
            angle: BigEndian::read_f32(&data[8..12])
        }
    }
    fn get_size() -> usize {
        12
    }
    fn get_buf_size() -> usize {
        10
    }
}