from read_txt import read_log
import cv2
import pandas as pd
from datetime import datetime,timedelta
from tqdm import tqdm
from PIL import Image
import numpy as np
import os

back_camera_timetable=[
    "2024-06-22 04:14:46.014936+09:00",#  1st
    "2024-06-22 04:18:54.096160+09:00",#  2nd
    "2024-06-22 04:41:42.115147+09:00",#  3rd
    "2024-06-22 05:04:31.035909+09:00",#  4th
    "2024-06-22 05:19:13.235154+09:00",#  5th
    "2024-06-22 05:35:07.393372+09:00",#  6th
    "2024-06-22 05:46:00.060000+09:00",#  7th
    "2024-06-22 06:18:36.235154+09:00",#  8th
    "2024-06-22 06:30:28.873671+09:00" #  9th
]

menu=[
    "滑走",     #  1st
    "短距離",   #  2nd
    "中距離",   #  3rd
    "中距離",   #  4th
    "中距離",   #  5th
    "飛び切り", #  6th
    "飛び切り", #  7th
    "飛び切り", #  8th
    "飛び切り", #  9th
]

num2kanji = [
    "一",
    "二",
    "三",
    "四",
    "五",
    "六",
    "七",
    "八",
    "九",
    "十"
]

i=8
target=f'video/7-{i}-back.mov'
output=f'output/7-{i}-back.mp4'
start=datetime.fromisoformat(back_camera_timetable[i-1])
date=f'{start.month:02}{start.day:02}'


CTRL_WIDTH,CTRL_HEIGHT = 400,150
def draw_ctrl(ctrl_img,df):
    ctrl_img[:,:,:]=32
    rudder=df['rudder_10'].iloc[-1]
    elevator=df['elevator_10'].iloc[-1]
    trim=df['trim_10'].iloc[-1]
    cv2.arrowedLine(ctrl_img,(75,75),(75+int(5*rudder),75+int(5*elevator)),(0, 255, 0),thickness=3)
    cv2.drawMarker(ctrl_img,(75+int(5*rudder),75+int(5*elevator)),(255,255,255),cv2.MARKER_CROSS,markerSize=10,thickness=1)
    cv2.circle(ctrl_img,(75,75),5*5, (255,255,255),1)
    cv2.circle(ctrl_img,(75,75),5*10,(255,255,255),1)
    cv2.circle(ctrl_img,(75,75),5*15,(255,255,255),1)
    cv2.putText(ctrl_img,
                f"rudder : {rudder:2.2f}",
                (160,50),
                fontFace = cv2.FONT_HERSHEY_DUPLEX, 
                fontScale = 0.8,
                color = (255, 255, 255), 
                thickness = 1
    )
    cv2.putText(ctrl_img,
                f"elevator : {elevator:2.2f}",
                (160,80),
                fontFace = cv2.FONT_HERSHEY_DUPLEX, 
                fontScale = 0.8, 
                color = (255, 255, 255), 
                thickness = 1
    )
    cv2.putText(ctrl_img,
                f"trim : {trim:2.2f}",
                (160,110),
                fontFace = cv2.FONT_HERSHEY_DUPLEX, 
                fontScale = 0.8, 
                color = (255, 255, 255), 
                thickness = 1
    )

PWR_WIDTH,PWR_HEIGHT=200,150
def draw_pwr(pwr_img,df):
    pwr_img[:,:,:]=32
    cadence=abs(df['cadence_21'].iloc[-1])
    # power=df['rudder_21'].iloc[-1]
    cv2.ellipse(pwr_img,(90,100),(80,80),181,0,int(cadence)+1,(0,0,255),thickness=10)
    cv2.putText(pwr_img,
                "CADENCE",
                (40,60),
                fontFace = cv2.FONT_HERSHEY_DUPLEX, 
                fontScale = 0.5, 
                color = (255, 255, 255), 
                thickness = 1
    )
    cv2.putText(pwr_img,
                f"{cadence:3.2f}",
                (30,100),
                fontFace = cv2.FONT_HERSHEY_DUPLEX, 
                fontScale = 1.0, 
                color = (255, 255, 255), 
                thickness = 1
    )


AIR_WIDTH,AIR_HEIGHT=200,200
def draw_air(pwr_img,df):
    pwr_img[:,:,:]=32
    ias=abs(df['velocity_31'].iloc[-1])
    cv2.ellipse(pwr_img,(90,100),(80,80),181,0,int(ias*10)+1,(255,128,64),thickness=10)
    cv2.putText(pwr_img,
                "AIR SPEED",
                (40,60),
                fontFace = cv2.FONT_HERSHEY_DUPLEX, 
                fontScale = 0.5, 
                color = (255, 255, 255), 
                thickness = 1
    )
    cv2.putText(pwr_img,
                f"{ias:2.2f}",
                (40,100),
                fontFace = cv2.FONT_HERSHEY_DUPLEX, 
                fontScale = 1.0, 
                color = (255, 255, 255), 
                thickness = 1
    )

ALT_WIDTH,ALT_HEIGHT=400,120
def draw_alt(alt_img,df):
    alt_img[:,:,:]=32
    alt=df['alt_52'].iloc[-1]
    P0=df['pressure_90'][(df["jst"]>start-timedelta(seconds=10))].iloc[0]
    p=df['pressure_90'].iloc[-1]
    alt_est=max(44330 * (1.0 - (p / P0)** 0.1903),0)
    
    cv2.rectangle(alt_img,(0,ALT_HEIGHT-int(alt*20)),(ALT_WIDTH//2,ALT_HEIGHT),color=(0,0,255),thickness=-1)

    cv2.rectangle(alt_img,(ALT_WIDTH//2,ALT_HEIGHT-int(alt_est*20)+1),(ALT_WIDTH,ALT_HEIGHT),color=(0,255,0),thickness=-1)

    cv2.putText(alt_img,
                "ULTRA SONIC",
                (30,ALT_HEIGHT-100),
                fontFace = cv2.FONT_HERSHEY_DUPLEX, 
                fontScale = 0.5,
                color = (255, 255, 255), 
                thickness = 1
    )
    cv2.putText(alt_img,
                f"{alt:2.2f} m",
                (30,ALT_HEIGHT-20),
                fontFace = cv2.FONT_HERSHEY_DUPLEX, 
                fontScale = 1.0,
                color = (255, 255, 255), 
                thickness = 1
    )
    
    cv2.putText(alt_img,
                "PRESSURE",
                (ALT_WIDTH//2+60,ALT_HEIGHT-100),
                fontFace = cv2.FONT_HERSHEY_DUPLEX, 
                fontScale = 0.5,
                color = (255, 255, 255), 
                thickness = 1
    )
    cv2.putText(alt_img,
                f"{p/100:4.2f} hPa",
                (ALT_WIDTH//2+50,ALT_HEIGHT-60),
                fontFace = cv2.FONT_HERSHEY_DUPLEX, 
                fontScale = 0.5,
                color = (255, 255, 255), 
                thickness = 1
    )
    cv2.putText(alt_img,
                f"{alt_est:2.2f} m",
                (ALT_WIDTH//2+50,ALT_HEIGHT-20),
                fontFace = cv2.FONT_HERSHEY_DUPLEX, 
                fontScale = 1.0,
                color = (255, 255, 255), 
                thickness = 1
    )


MAP_ZOOM,MAP_X,MAP_Y=14,14541,6434


if __name__=='__main__':


    from sensor import ServoController,Vane,Altimeter,Pitot,Tachometer,GPS,Barometer
    df_all=read_log(date,ServoController())
    df_all=pd.merge(df_all,read_log(date,Vane(0x71)),how='outer',on=['utc','jst'])
    df_all=pd.merge(df_all,read_log(date,Altimeter(0x52)),how='outer',on=['utc','jst'])
    df_all=pd.merge(df_all,read_log(date,Tachometer(0x21)),how='outer',on=['utc','jst'])
    df_all=pd.merge(df_all,read_log(date,Pitot(0x31)),how='outer',on=['utc','jst'])
    df_all=pd.merge(df_all,read_log(date,Barometer(0x90)),how='outer',on=['utc','jst'])
    df_all=pd.merge(df_all,read_log(date,GPS(0x60)),how='outer',on=['utc','jst'])

    movie=cv2.VideoCapture(target)
    fps=movie.get(cv2.CAP_PROP_FPS)
    height= int(movie.get(cv2.CAP_PROP_FRAME_HEIGHT))
    width = int(movie.get(cv2.CAP_PROP_FRAME_WIDTH))
    cnt=int(movie.get(cv2.CAP_PROP_FRAME_COUNT))


    df_all=df_all.sort_values('utc')
    df_all=df_all.interpolate()


    print(f"    {start}    --( {cnt/fps} sec)-->    {start+timedelta(seconds=cnt/fps)} ")
    codec=cv2.VideoWriter.fourcc(*'mp4v')
    video= cv2.VideoWriter('tmp1.mp4', codec, fps, (width+400, height))
    
    image = np.zeros( (height, width+400,3), dtype=np.uint8 )
    

    map_img = cv2.imread(f'../assets/map/{MAP_ZOOM}-{MAP_X}-{MAP_Y}.png')
    ctrl_img = np.zeros((CTRL_HEIGHT,CTRL_WIDTH,3),dtype=np.uint8)
    pwr_img = np.zeros((PWR_HEIGHT,PWR_WIDTH,3),dtype=np.uint8)
    air_img = np.zeros((AIR_HEIGHT,AIR_WIDTH,3),dtype=np.uint8)
    alt_img = np.zeros((ALT_HEIGHT,ALT_WIDTH,3),dtype=np.uint8)

    font_scale=0.5

    for t in tqdm(range(cnt)):
        image[:,:,:]=32
        rest,cap=movie.read()
        if not rest:
            break
        image[:height,:width,:]=cap
        df=df_all[df_all['jst']<=start+timedelta(seconds=t/fps)]

        
        cv2.putText(
            image,
            text=f'{start+timedelta(seconds=t/fps)}',
            org=(int(width+55*font_scale), int(height - 55//2 * font_scale)),
            fontFace = cv2.FONT_HERSHEY_DUPLEX, 
            fontScale = 0.5, 
            color = (255, 255, 255), 
            thickness = 1, 
        )
        draw_ctrl(ctrl_img,df)
        draw_pwr(pwr_img,df)
        draw_air(air_img,df)
        draw_alt(alt_img,df)

        x=int((2.0**(MAP_ZOOM+7.0))*(df['lon_60'].iloc[-1]/180.0+1))
        y=int((2.0**(MAP_ZOOM+7.0))/np.pi*(-np.arctanh(np.sin(np.radians(df['lat_60'].iloc[-1]))) + np.arctanh(np.sin(np.radians(85.05112878)))))
        if ((x//256)!=MAP_X) or ((y//256)!=MAP_Y):
            MAP_X=x//256
            MAP_Y=y//256
            map_img=cv2.imread(f'../assets/map/{MAP_ZOOM}-{MAP_X}-{MAP_Y}.png')
        cv2.circle(map_img, (x%256,y%256), 2, (0, 0, 255), thickness=-1)

        image[:256,:256,:]=map_img

        cursor_x,cursor_y=width,50
        image[cursor_y:cursor_y+CTRL_HEIGHT,cursor_x:cursor_x+CTRL_WIDTH,:]=ctrl_img
        cursor_y+=(CTRL_HEIGHT+100)
        image[cursor_y:cursor_y+PWR_HEIGHT ,cursor_x:cursor_x+PWR_WIDTH,:]=pwr_img
        cursor_x+=(PWR_WIDTH)
        image[cursor_y:cursor_y+AIR_HEIGHT ,cursor_x:cursor_x+AIR_WIDTH,:]=air_img
        cursor_y+=(max(PWR_HEIGHT,AIR_HEIGHT)+100)
        cursor_x=width
        image[cursor_y:cursor_y+ALT_HEIGHT ,cursor_x:cursor_x+ALT_WIDTH,:]=alt_img

        video.write(image)
    
    video.release()
    os.system(f'ffmpeg -i {target} tmp.wav')
    os.system('ffmpeg -i tmp1.mp4  -i tmp.wav -c:v copy -c:a aac tmp2.mp4')
    os.system(f'ffmpeg -i tmp2.mp4 {output}')
    os.system('rm tmp*.*')