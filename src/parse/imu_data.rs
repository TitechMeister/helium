use crate::parse::Data;
use byteorder::{BigEndian, ByteOrder};

#[derive(Debug, Clone, Copy)]
pub struct IMUData{
    pub id: u8,
    pub timestamp: u32,
    pub q_w: i16,
    pub q_x: i16,
    pub q_y: i16,
    pub q_z: i16
}

impl Data for IMUData{
    fn parse(data: &Vec<u8>) -> Self{
        IMUData{
            id: data[0],
            timestamp: BigEndian::read_u32(&data[4..8]),
            q_w: BigEndian::read_i16(&data[8..10]),
            q_x: BigEndian::read_i16(&data[10..12]),
            q_y: BigEndian::read_i16(&data[12..14]),
            q_z: BigEndian::read_i16(&data[14..16]),
        }
    }
    fn get_size() -> usize {
        16
    }
}