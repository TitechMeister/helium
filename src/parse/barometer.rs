use crate::parse::Data;
use byteorder::{BigEndian, ByteOrder};


#[derive(Debug, Clone, Copy)]
pub struct BarometerData{
    pub id: u8,
    pub timestamp: u32,
    pub pressure:f32,
    pub tempareture:f32
}

impl Data for BarometerData{
    fn parse(data: &Vec<u8>) -> Self{
        BarometerData{
            id: data[0],
            timestamp: BigEndian::read_u32(&data[4..8]),
            pressure: BigEndian::read_f32(&data[8..12]),
            tempareture: BigEndian::read_f32(&data[12..16])
        }
    }
    fn get_size() -> usize {
        16
    }
}