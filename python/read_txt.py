from sensor import Sensor
import pandas as pd
from datetime import datetime,timezone,timedelta

def read_log(date:str,id:int,sensor:Sensor):
    with open(f'log/{date}/log.bin-id{id}.txt') as f:
        lines=f.readlines()
    utc=[]
    for l in lines:
        utc0=int(l[:13])
        n=sensor.parse([int(s) for s in l[15:-2].split(',')])
        for i in range(n,0,-1):
            utc.append(utc0-(sensor.database[f'timestamp_{sensor.id:02x}'][-1]-sensor.database[f'timestamp_{sensor.id:02x}'][-i]))
    print(f'sensor{sensor.ID:02x} : {len(utc)} items')
    df=pd.DataFrame(sensor.database)
    df['utc']=utc
    JST = timezone(timedelta(hours=+9))
    df['jst']=[datetime.fromtimestamp(t/1000.0,timezone.utc).astimezone(JST) for t in utc]
    return df

if __name__=='__main__':
    from sensor import Altimeter
    import matplotlib.pyplot as plt
    sensor=Altimeter()
    df=read_log('0608',82,sensor)
    # 2024/06/08 4:51 から3分間で取得したデータを切り出す。
    JST = timezone(timedelta(hours=+9))
    start=datetime(2024,6,8,5,4,51,tzinfo=JST)
    end=  start + timedelta(seconds=180)
    df_flight=df[(df['jst']>start)&(df['jst']<end)]
    plt.plot(df_flight['jst'],df_flight['alt'])
    plt.show()