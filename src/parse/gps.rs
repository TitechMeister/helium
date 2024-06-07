use crate::parse::Data;
use byteorder::{BigEndian, ByteOrder};

#[derive(Debug, Clone, Copy)]
pub struct GPSData{
    pub id: u8,
    pub timestamp: u32,
    pub longitude: f64,
    pub latitude: f64
}

impl Data for GPSData{
    fn parse(data: &Vec<u8>) -> Self{
        GPSData{
            id: data[0],
            timestamp: BigEndian::read_u32(&data[4..8]),
            longitude: BigEndian::read_f64(&data[8..16]),
            latitude: BigEndian::read_f64(&data[16..24])
        }
    }
    fn get_size() -> usize {
        24
    }
}