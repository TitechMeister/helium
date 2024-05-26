use log::info;
use std::fs::OpenOptions;
use std::io::prelude::*;
use crate::parse::{decode_cobs, AltData, Data, IMUData, ServoData,PitotData};
use std::fmt::Debug;



fn parse_data<T>(data: &mut Vec<T>,decoded: &Vec<u8>)
where T: Data+Debug+Copy+Clone{
    for i in (0..decoded.len()).step_by(T::get_size()){
        let item = T::parse(&decoded[i..i+T::get_size()].to_vec());
        info!("{:?}",item);
        data.push(item);
    }
    if data.len() > 100 {
        *data = data[data.len()-100..].to_vec();
    }
}

pub struct Parser {
    log: Vec<u8>,
    pub filename: String,
    imu: [Vec<IMUData>;3],
    servo_data: Vec<ServoData>,
    alt_data: Vec<AltData>,
    pitot_data:Vec<PitotData>,
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

    pub fn get_imu(&self,id:u8) -> &Vec<IMUData> {
        &self.imu[(id&0x0f) as usize]
    }

    pub fn get_servo_data(&self) -> &Vec<ServoData> {
        &self.servo_data
    }

    pub fn get_alt_data(&self) -> &Vec<AltData> {
        &self.alt_data
    }

    pub fn get_pitot_data(&self)->&Vec<PitotData>{
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
                        let mut file = OpenOptions::new()
                            .write(true)
                            .append(true)
                            .create(true)
                            .open(format!("{}.txt", self.filename))
                            .unwrap();
                        let timestamp = chrono::Local::now().timestamp_millis();
                        file.write_all(format!("{}:", timestamp).as_bytes()).unwrap();
                        file.write_all(format!("{:?}",decoded).as_bytes()).unwrap();
                        file.write_all("\n".as_bytes()).unwrap();
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
