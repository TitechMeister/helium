from read_txt import read_log
import cv2
import pandas as pd
from datetime import datetime,timezone,timedelta
from tqdm import tqdm
import numpy as np

if __name__=='__main__':
    from sensor import ServoController,Vane,Altimeter,Pitot,Tachometer,GPS,Barometer
    servo=ServoController()
    df=read_log('0608',16,servo)
    df=pd.merge(df,read_log('0608',0x71,Vane()),how='outer',on=['utc','jst'])
    df=pd.merge(df,read_log('0608',0x52,Altimeter()),how='outer',on=['utc','jst'])
    df=pd.merge(df,read_log('0608',0x21,Tachometer()),how='outer',on=['utc','jst'])
    df=pd.merge(df,read_log('0608',0x31,Pitot()),how='outer',on=['utc','jst'])
    df=pd.merge(df,read_log('0608',0x90,Barometer()),how='outer',on=['utc','jst'])
    df=pd.merge(df,read_log('0608',0x06,GPS()),how='outer',on=['utc','jst'])

    # 2024/06/08 4:51 から3分間で取得したデータを切り出す。
    JST = timezone(timedelta(hours=+9))
    start=datetime(2024,6,8,5,5,52,587,tzinfo=JST)
    end=  start + timedelta(seconds=180)


    df=df[(df['jst']>=start)&(df['jst']<=end)]

    df=df.sort_values('utc')


    print(f"start={df['jst'].iloc[0]}")
    print(f"end={df['jst'].iloc[-1]}")

    width,height=1920,1080
    fps=29.97
    codec=cv2.VideoWriter.fourcc(*'mp4v')
    video= cv2.VideoWriter('6thTF-4thFlight.mp4', codec, fps, (width, height))

    data={
        'rudder':0.0,
        'elevator':0.0,
        'trim':0.0,
        'cadence':0.0,
        'alt':0.0,
        'lat':0.0,
        'lon':0.0,
        'angle':0.0,
        'pressure':0.0,
        'temperature':0.0,
    }

    
    for t in tqdm(range(int(fps*(end-df['jst'].iloc[1]).seconds))):
        image = np.full( (height, width,3), (0, 0, 0), dtype=np.uint8 )
        for k,key in enumerate(data):
            _df=df[df['jst']<start+timedelta(seconds=t/fps)]
            if len(_df)==0:
                continue
            if not np.isnan(_df[key].iloc[-1]):
                data[key]=_df[key].iloc[-1]
            cv2.putText(
                    image,
                    text=f'{key}:{data[key]:3.3f}',
                    org=(10, 55 + 65 * k),
                    fontFace = cv2.FONT_HERSHEY_DUPLEX, 
                    fontScale = 1.0, 
                    color = (255, 255, 255), 
                    thickness = 1, 
            )
        cv2.putText(
            image,
            text=f'{start+timedelta(seconds=t/fps)}',
            org=(10, height - 65 * 1),
            fontFace = cv2.FONT_HERSHEY_DUPLEX, 
            fontScale = 0.5, 
            color = (255, 255, 255), 
            thickness = 1, 
        )
        video.write(image)
    
    video.release()