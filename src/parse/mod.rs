mod cobs;

mod ultra_sonic;
mod barometer;
mod gps;
mod imu;
mod pitot;
mod servo;
mod tachometer;
mod vane;

mod parser;

pub use cobs::decode_cobs;
pub use barometer::BarometerData;
pub use servo::ServoData;
pub use ultra_sonic::UltraSonicData;
pub use gps::GPSData;
pub use imu::IMUData;
pub use pitot::PitotData;
pub use vane::VaneData;
pub use tachometer::TachData;
pub use parser::Parser;

pub trait Data{
    fn parse(data: &Vec<u8>) -> Self;
    fn get_size() -> usize;
}

