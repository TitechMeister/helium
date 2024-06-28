import fitdecode
from zoneinfo import ZoneInfo
import csv

datas = []
target='./24-7th8full.fit'
with fitdecode.FitReader(target) as fit:
    for frame in fit:

        if isinstance(frame, fitdecode.FitDataMessage):
            if frame.name == 'record':
                data = {
                    'right_pedal_smoothness':0.0,
                    'distance':0.0,
                    'left_power_phase':0.0,
                    'right_power_phase_peak':0.0,
                    'temperature':0.0,
                    'position_lat':0.0,
                    'left_torque_effectiveness':0.0,
                    'unknown_61':0.0,
                    'heart_rate':0.0,
                    'left_right_balance':0.0,
                    'enhanced_speed':0.0,
                    'right_pco':0.0,
                    'right_power_phase':0.0,
                    'timestamp':'',
                    'left_pco':0.0,
                    'right_torque_effectiveness':0.0,
                    'enhanced_altitude':0.0,
                    'left_pedal_smoothness':.0,
                    'fractional_cadence':.0,
                    'unknown_66':None,
                    'accumulated_power':None,
                    'left_power_phase_peak':None,
                    'position_long':None,
                    'altitude':None,
                    'speed':None,
                    'power':None,
                    'cadence':None
                }
                for field in frame.fields:
                    data[f'{field.name}'] = field.value
                data['jtc']=data['timestamp'].astimezone(ZoneInfo('Asia/Tokyo'))
                datas.append(data)

with open(f'{target}.csv', mode='w', newline='') as f:
    writer = csv.DictWriter(f, datas[0].keys())
    writer.writeheader()
    writer.writerows(datas)
