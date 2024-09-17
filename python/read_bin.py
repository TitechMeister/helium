from cobs import cobs_decode
from sensor import Altimeter,IMU,Pitot,ServoController,Tachometer,GPS,Vane,Barometer
from tqdm import tqdm
import pandas as pd

if __name__=="__main__":
    with open("../log.bin","rb") as f:
        data = f.read()
    servo_controller = ServoController()
    altimeter = Altimeter()
    imu40 = IMU()
    imu41 = IMU()
    imu42 = IMU()
    pitot = Pitot()
    tachometer = Tachometer()
    gps=GPS()
    vane=Vane()
    barometer=Barometer()
    with tqdm(total=len(data)+1) as pbar:
        while len(data)>0:
            before = len(data)
            dec, data = cobs_decode(data)
            pbar.update(before-len(data))
            if len(dec)==0: # Empty packet
                continue
            match dec[0] & 0xF0:
                case 0x10:
                    servo_controller.parse(dec)
                case 0x20:
                    tachometer.parse(dec)
                case 0x30:
                    pitot.parse(dec)
                case 0x40:
                    match dec[0]&0x0F:
                        case 0x00:
                            imu40.parse(dec)
                        case 0x01:
                            imu41.parse(dec)
                        case 0x02:
                            imu42.parse(dec)
                case 0x50:
                    altimeter.parse(dec)
                case 0x60:
                    gps.parse(dec)
                case 0x70:
                    vane.parse(dec)
                case 0x90:
                    barometer.parse(dec)

    df_alt = pd.DataFrame(altimeter.raw_data)
    df_imu40 = pd.DataFrame(imu40.raw_data)
    df_imu41 = pd.DataFrame(imu41.raw_data)
    df_imu42 = pd.DataFrame(imu42.raw_data)
    df_pitot = pd.DataFrame(pitot.raw_data)
    df_servo_controller = pd.DataFrame(servo_controller.raw_data)
    df_tachometer = pd.DataFrame(tachometer.raw_data)
    df_gps = pd.DataFrame(gps.raw_data)
    df_vane = pd.DataFrame(vane.raw_data)
    df_barometer = pd.DataFrame(barometer.raw_data)
    
    df_alt.to_csv("log/altimeter.csv")
    df_imu40.to_csv("log/imu40.csv")
    df_imu41.to_csv("log/imu41.csv")
    df_imu42.to_csv("log/imu42.csv")
    df_pitot.to_csv("log/pitot.csv")
    df_servo_controller.to_csv("log/servo_controller.csv")
    df_tachometer.to_csv("log/tachometer.csv")
    df_gps.to_csv("log/gps.csv")
    df_vane.to_csv("log/vane.csv")
    df_barometer.to_csv("log/barometer.csv")