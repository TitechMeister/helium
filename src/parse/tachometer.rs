use crate::parse::Data;
use byteorder::{BigEndian, ByteOrder};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct TachData{
    pub id: u8,
    pub timestamp: u32,
    pub cadence:f64,
    pub strain:u32
}

impl Data for TachData{
    fn parse(data: &Vec<u8>) -> Self{
        TachData{
            id: data[0],
            timestamp: BigEndian::read_u32(&data[4..8]),
            cadence: BigEndian::read_f64(&data[8..16]),
            strain: BigEndian::read_u32(&data[16..20]),
        }
    }
    fn get_size() -> usize {
        24
    }
    fn get_buf_size() -> usize {
        0
    }
}