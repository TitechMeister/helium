use log::info;
use std::fs::OpenOptions;
use std::io::prelude::*;
use crate::parse::{decode_cobs, AltData, Data, IMUData, ServoData};

use super::PitotData;

pub struct Parser {
    log: Vec<u8>,
    pub filename: String,
    imu: [Vec<Box<IMUData>>;3],
    servo_data: Vec<Box<ServoData>>,
    alt_data: Vec<Box<AltData>>,
    pitot_data:Vec<Box<PitotData>>,
    port: Option<Box<dyn serialport::SerialPort>>,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            log: Vec::new(),
            filename : "log.bin".to_owned(),
            imu: [Vec::new(),Vec::new(),Vec::new()],
            servo_data: Vec::new(),
            alt_data: Vec::new(),
            pitot_data:Vec::new(),
            port: None,
        }
    }

    pub fn get_port(&self) -> &Option<Box<dyn serialport::SerialPort>> {
        &self.port
    }

    pub fn set_port(&mut self, mut port: Box<dyn serialport::SerialPort>) {
        match self.port {
            Some(_) => {}
            None => {
                port.set_timeout(std::time::Duration::from_millis(10))
                    .unwrap();
                self.port = Some(port);
            }
        }
    }

    pub fn get_imu(&self,id:u8) -> &Vec<Box<IMUData>> {
        &self.imu[(id&0x0f) as usize]
    }

    pub fn get_servo_data(&self) -> &Vec<Box<ServoData>> {
        &self.servo_data
    }

    pub fn get_alt_data(&self) -> &Vec<Box<AltData>> {
        &self.alt_data
    }

    pub fn get_pitot_data(&self)->&Vec<Box<PitotData>>{
        &self.pitot_data
    }

    #[allow(unused_assignments)]
    pub fn parse(&mut self) {
        let mut serial_buf = [0; 1024];
        match self.port {
            Some(ref mut port) => match port.read(serial_buf.as_mut_slice()) {
                Ok(n) => {
                    log::info!("recv:{:?}", &serial_buf[..n]);

                    let mut file = OpenOptions::new()
                        .write(true)
                        .append(true)
                        .create(true)
                        .open(&self.filename)
                        .unwrap();
                    file.write_all(&serial_buf[..n]).unwrap();

                    self.log.extend(serial_buf[..n].iter());
                    let mut decoded: Vec<u8> = vec![];
                    let mut rest: Vec<u8> = vec![];
                    (decoded, rest) = decode_cobs(&self.log);
                    while decoded.len() > 0 {
                        log::info!("{:?}", decoded);
                        match decoded[0] & 0xF0 {
                            0x40 => {
                                for i in 0..(decoded.len() / 16){
                                    self.imu[(decoded[0]&0x0F) as usize].push(Box::new(IMUData::parse(&decoded[i*16..(i+1)*16].to_vec())));
                                }
                                if self.imu[(decoded[0]&0x0F) as usize].len() > 100 {
                                    self.imu[(decoded[0]&0x0F) as usize] = self.imu[(decoded[0]&0x0F) as usize][self.imu[(decoded[0]&0x0F) as usize].len()-100..].to_vec();
                                }
                            }
                            0x10 => {
                                self.servo_data
                                    .push(Box::new(ServoData::parse(&decoded.to_vec())));
                                info!("{:?}", self.servo_data.last().unwrap());
                                if self.servo_data.len() > 100 {
                                    self.servo_data = self.servo_data[self.servo_data.len()-100..].to_vec();
                                }
                            }
                            0x30 => {
                                self.pitot_data
                                    .push(Box::new(PitotData::parse(&decoded.to_vec())));
                                info!("{:?}", self.pitot_data.last().unwrap());
                                if self.pitot_data.len() > 100 {
                                    self.pitot_data = self.pitot_data[self.pitot_data.len()-100..].to_vec();
                                }
                            }
                            0x50 => {
                                self.alt_data
                                    .push(Box::new(AltData::parse(&decoded.to_vec())));
                                info!("{:?}", self.alt_data.last().unwrap());
                                if self.alt_data.len() > 100 {
                                    self.alt_data = self.alt_data[self.alt_data.len()-100..].to_vec();
                                }
                            }
                            _ => (),
                        }
                        self.log = rest.to_vec();
                        (decoded, rest) = decode_cobs(&self.log);
                    }
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => (),
                Err(e) => {
                    log::error!("{:?}", e);
                    self.port = None;
                },
            },
            None => (),
        }
    }
}
