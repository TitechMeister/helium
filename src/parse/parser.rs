use std::fs::OpenOptions;
use std::io::prelude::*;
use crate::parse::{decode_cobs, UltraSonicData,BarometerData,GPSData, Data, IMUData, ServoData,PitotData,TachData};
use std::fmt::Debug;

use super::VaneData;



fn parse_data<T>(data: &mut Vec<(T,i64)>,decoded: &Vec<u8>,timestamp:i64)
where T: Data+Debug+Copy+Clone{
    if decoded.len() < T::get_size() {
        return;
    }
    for i in 0..decoded.len()/T::get_size(){
        let item = T::parse(&decoded[i*T::get_size()..(i+1)*T::get_size()].to_vec());
        println!("{:?}",item);
        data.push((item,timestamp));
    }
    if data.len() > 100 {
        *data = data[data.len()-100..].to_vec();
    }
}

pub struct Parser {
    log: Vec<u8>,
    pub filename: String,
    gps_data: Vec<(GPSData,i64)>,
    imu: [Vec<(IMUData,i64)>;16],
    servo_data: Vec<(ServoData,i64)>,
    alt_data: Vec<(UltraSonicData,i64)>,
    barometer_data: [Vec<(BarometerData,i64)>;2],
    pitot_data:Vec<(PitotData,i64)>,
    vane_data:Vec<(VaneData,i64)>,
    tac_data: [Vec<(TachData,i64)>;2],
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
            barometer_data: [Vec::new(),Vec::new()],
            pitot_data:Vec::new(),
            gps_data: Vec::new(),
            vane_data: Vec::new(),
            tac_data: [Vec::new(),Vec::new()],
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

    pub fn get_imu(&self,id:u8) -> &Vec<(IMUData,i64)> {
        &self.imu[(id&0x0f) as usize]
    }

    pub fn get_servo_data(&self) -> &Vec<(ServoData,i64)> {
        &self.servo_data
    }

    pub fn get_ultra_sonic_data(&self) -> &Vec<(UltraSonicData,i64)> {
        &self.alt_data
    }

    pub fn get_barometer_data(&self,id:u8) -> &Vec<(BarometerData,i64)> {
        if id==0{
            &self.barometer_data[0]
        }else{
            &self.barometer_data[1]
        }
    }

    pub fn get_pitot_data(&self)->&Vec<(PitotData,i64)>{
        &self.pitot_data
    }
    pub fn get_gps_data(&self) -> &Vec<(GPSData,i64)> {
        &self.gps_data
    }

    pub fn get_vane_data(&self) -> &Vec<(VaneData,i64)>{
        &self.vane_data
    }

    pub fn get_tach_data(&self,id:u8) -> &Vec<(TachData,i64)> {
        &self.tac_data[(id&0x01) as usize]
    }

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
                    let (mut decoded, mut rest) = decode_cobs(&self.log);
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
                                parse_data(&mut self.servo_data,&decoded,timestamp);
                            }
                            0x20 => {
                                if decoded[0] == 0x20 {
                                    parse_data(&mut self.tac_data[0],&decoded,timestamp);
                                }else if decoded[0]==0x21 {
                                    parse_data(&mut self.tac_data[1],&decoded,timestamp);
                                }
                            }
                            0x30 => {
                                parse_data(&mut self.pitot_data,&decoded,timestamp);
                            }
                            0x40 => {
                                parse_data(&mut self.imu[(decoded[0] & 0x0f) as usize], &decoded,timestamp);
                            }
                            0x50 => {
                                parse_data(&mut self.alt_data,&decoded,timestamp);
                            }
                            0x60 => {
                                parse_data(&mut self.gps_data,&decoded,timestamp);
                            }
                            0x70 => {
                                parse_data(&mut self.vane_data, &decoded,timestamp);
                            }
                            0x90 => {
                                if decoded[0] == 0x90{
                                    parse_data(&mut self.barometer_data[0], &decoded,timestamp);
                                }else{
                                    parse_data(&mut self.barometer_data[1], &decoded,timestamp);
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
                    println!("{:?}", e);
                    self.port = None;
                },
            },
            None => (),
        }
    }
}
