from read_txt import read_log
import cv2
import pandas as pd
from datetime import datetime,timezone,timedelta
from tqdm import tqdm
from PIL import Image
import numpy as np
import os

if __name__=='__main__':

    target='target.mp4'
    date='0608'
    JST = timezone(timedelta(hours=+9))
    start=datetime(2024,6,8,5,6,14,tzinfo=JST)-timedelta(seconds=24)

    MAP_ZOOM,MAP_X,MAP_Y=14,14541,6434

    from sensor import ServoController,Vane,Altimeter,Pitot,Tachometer,GPS,Barometer
    df=read_log(date,0x10,ServoController())
    df=pd.merge(df,read_log(date,0x71,Vane(0x71)),how='outer',on=['utc','jst'])
    df=pd.merge(df,read_log(date,0x52,Altimeter(0x52)),how='outer',on=['utc','jst'])
    df=pd.merge(df,read_log(date,0x21,Tachometer(0x31)),how='outer',on=['utc','jst'])
    df=pd.merge(df,read_log(date,0x31,Pitot(0x21)),how='outer',on=['utc','jst'])
    df=pd.merge(df,read_log(date,0x90,Barometer(0x90)),how='outer',on=['utc','jst'])
    df=pd.merge(df,read_log(date,0x06,GPS(0x06)),how='outer',on=['utc','jst'])

    movie=cv2.VideoCapture(target)
    fps=movie.get(cv2.CAP_PROP_FPS)
    height= int(movie.get(cv2.CAP_PROP_FRAME_HEIGHT))
    width = int(movie.get(cv2.CAP_PROP_FRAME_WIDTH))
    cnt=int(movie.get(cv2.CAP_PROP_FRAME_COUNT))


    df=df.sort_values('utc')
    df=df.interpolate()


    print(f"start={df['jst'].iloc[0]}")
    print(f"end={df['jst'].iloc[-1]}")
    codec=cv2.VideoWriter.fourcc(*'mp4v')
    video= cv2.VideoWriter('tmp1.mp4', codec, fps, (width+400, height))

    data={
        'rudder_10':0.0,
        'elevator_10':0.0,
        'trim_10':0.0,
        'cadence_31':0.0,
        'alt_52':0.0,
        'lat_06':0.0,
        'lon_06':0.0,
        'angle_71':0.0,
        'pressure_90':0.0,
        'temperature_90':0.0,
    }

    
    image = np.zeros( (height, width+400,3), dtype=np.uint8 )
    
    # url=f"{https://tile.openstreetmap.org/{z}/{x}/{y}.png}" # OpenStreatMap
    # url=f"{http://cyberjapandata.gsi.go.jp/xyz/std/{z}/{x}/{y}.png}" # 国土地理院
    map_img = cv2.imread(f'../assets/map/{MAP_ZOOM}-{MAP_X}-{MAP_Y}.png')

    for t in tqdm(range(cnt)):
        image[:,:,:]=32
        _,cap=movie.read()
        image[:height,:width,:]=cap
        _df=df[df['jst']<=start+timedelta(seconds=t/fps)]
        for k,key in enumerate(data):
            data[key]=_df[key].iloc[-1]
            cv2.putText(
                    image,
                    text=f'{key[:-3]} : {data[key]:3.3f}',
                    org=(width+55, 55 + 65 * k),
                    fontFace = cv2.FONT_HERSHEY_DUPLEX, 
                    fontScale = 0.8, 
                    color = (255, 255, 255), 
                    thickness = 1, 
            )
        cv2.circle(map_img, 
                   (
                        int((2.0**(MAP_ZOOM+7.0))*(data['lon_06']/180.0+1))%256,
                        int((2.0**(MAP_ZOOM+7.0))/np.pi*(-np.arctanh(np.sin(np.radians(data['lat_06']))) + np.arctanh(np.sin(np.radians(85.05112878)))))%256
                    ), 1, (255, 0, 0), thickness=-1)
        
        cv2.putText(
            image,
            text=f'{start+timedelta(seconds=t/fps)}',
            org=(width+55, height - 55//2 * 1),
            fontFace = cv2.FONT_HERSHEY_DUPLEX, 
            fontScale = 0.5, 
            color = (255, 255, 255), 
            thickness = 1, 
        )
        image[:256,:256,:]=map_img
        video.write(image)
    
    video.release()
    os.system(f'ffmpeg -i {target} tmp.wav')
    os.system('ffmpeg -i tmp1.mp4 -i tmp.wav -c:v copy -c:a aac tmp2.mp4')
    os.system('ffmpeg -i tmp2.mp4 output.mp4')
    os.system('rm tmp*.*')