use std::fs::OpenOptions;
use std::io::prelude::*;
use crate::parse::{decode_cobs, AltData,BarometerData,GPSData, Data, IMUData, ServoData,PitotData};
use std::fmt::Debug;

use super::VaneData;



fn parse_data<T>(data: &mut Vec<T>,decoded: &Vec<u8>)
where T: Data+Debug+Copy+Clone{
    for i in 0..decoded.len()/T::get_size(){
        let item = T::parse(&decoded[i*T::get_size()..(i+1)*T::get_size()].to_vec());
        println!("{:?}",item);
        data.push(item);
    }
    if data.len() > 100 {
        *data = data[data.len()-100..].to_vec();
    }
}

pub struct Parser {
    log: Vec<u8>,
    pub filename: String,
    gps_data: Vec<GPSData>,
    imu: [Vec<IMUData>;16],
    servo_data: Vec<ServoData>,
    alt_data: Vec<AltData>,
    barometer_data: Vec<BarometerData>,
    pitot_data:Vec<PitotData>,
    vane_data:Vec<VaneData>,
    port: Option<Box<dyn serialport::SerialPort>>,
}

impl Parser {
    pub fn new() -> Self {
        std::fs::create_dir_all(format!("log/{}",chrono::Local::now().format("%m%d"))).unwrap();
        Self {
            log: Vec::new(),
            filename : "log.bin".to_owned(),
            imu: [  Vec::new(),Vec::new(),Vec::new(),Vec::new(),
                    Vec::new(),Vec::new(),Vec::new(),Vec::new(),
                    Vec::new(),Vec::new(),Vec::new(),Vec::new(),
                    Vec::new(),Vec::new(),Vec::new(),Vec::new() ],
            servo_data: Vec::new(),
            alt_data: Vec::new(),
            barometer_data: Vec::new(),
            pitot_data:Vec::new(),
            gps_data: Vec::new(),
            vane_data: Vec::new(),
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

    pub fn write(&mut self, buf: &Vec<u8>) {
        let cobs=crate::parse::cobs::encode_cobs(&buf);
        match self.port.as_mut() {
            Some(port) => {
                port.write(&cobs[..]).unwrap();
            }
            None => {}
        }
    }

    pub fn get_imu(&self,id:u8) -> &Vec<IMUData> {
        &self.imu[(id&0x0f) as usize]
    }

    pub fn get_servo_data(&self) -> &Vec<ServoData> {
        &self.servo_data
    }

    pub fn get_alt_data(&self) -> &Vec<AltData> {
        &self.alt_data
    }

    pub fn get_barometer_data(&self) -> &Vec<BarometerData> {
        &self.barometer_data
    }

    pub fn get_pitot_data(&self)->&Vec<PitotData>{
        &self.pitot_data
    }
    pub fn get_gps_data(&self) -> &Vec<GPSData> {
        &self.gps_data
    }

    pub fn get_vane_data(&self) -> &Vec<VaneData>{
        &self.vane_data
    }

    #[allow(unused_assignments)]
    pub fn parse(&mut self) {
        let mut serial_buf = [0; 1024];
        match self.port {
            Some(ref mut port) => match port.read(serial_buf.as_mut_slice()) {
                Ok(n) => {
                    let mut file = OpenOptions::new()
                        .write(true)
                        .append(true)
                        .create(true)
                        .open(&format!("log/{}/{}",chrono::Local::now().format("%m%d"),self.filename))
                        .unwrap();
                    file.write_all(&serial_buf[..n]).unwrap();

                    self.log.extend(serial_buf[..n].iter());
                    let mut decoded: Vec<u8> = vec![];
                    let mut rest: Vec<u8> = vec![];
                    (decoded, rest) = decode_cobs(&self.log);
                    while decoded.len() > 0 {
                        println!("{:?}", decoded);
                        let mut file = OpenOptions::new()
                            .write(true)
                            .append(true)
                            .create(true)
                            .open(format!("log/{}/{}-id{}.txt",chrono::Local::now().format("%m%d"), self.filename,&decoded[0]))
                            .unwrap();
                        let timestamp = chrono::Utc::now().timestamp_millis();
                        file.write_all(format!("{}:{:?}\n", timestamp,decoded).as_bytes()).unwrap();
                        match decoded[0] & 0xF0 {
                            0x10 => {
                                parse_data(&mut self.servo_data,&decoded);
                            }
                            0x30 => {
                                parse_data(&mut self.pitot_data,&decoded);
                            }
                            0x40 => {
                                parse_data(&mut self.imu[(decoded[0] & 0x0f) as usize], &decoded);
                            }
                            0x50 => {
                                parse_data(&mut self.alt_data,&decoded);
                            }
                            0x60 => {
                                parse_data(&mut self.gps_data,&decoded);
                            }
                            0x70 => {
                                parse_data(&mut self.vane_data, &decoded);
                            }
                            0x90 => {
                                parse_data(&mut self.barometer_data, &decoded);
                            }
                            _ => (),
                        }
                        self.log = rest.to_vec();
                        (decoded, rest) = decode_cobs(&self.log);
                    }
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => (),
                Err(e) => {
                    println!("{:?}", e);
                    self.port = None;
                },
            },
            None => (),
        }
    }
}
