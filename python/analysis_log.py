from read_txt import read_log
import pandas as pd
from datetime import datetime,timezone,timedelta
from matplotlib import pyplot as plt,gridspec
from PIL import Image
import numpy as np
from scipy.spatial import distance
import tqdm

if __name__=='__main__':
    from sensor import ServoController,Vane,Altimeter,Pitot,Tachometer,GPS,Barometer,IMU
    df=read_log('0608',Altimeter(0x52))
    
    JST = timezone(timedelta(hours=+9))
    start=datetime(2024,6,8,4,52,0,tzinfo=JST)-timedelta(seconds=0)
    end=  start + timedelta(seconds=60)
    df=df[(df['jst']>=start)&(df['jst']<=end)]
    z=df['alt_52'].to_numpy()
    jst=df['jst']
    # df=pd.merge(df,read_log('0608',Vane(0x71)),how='outer',on=['utc','jst'])
    # df=pd.merge(df,read_log('0608',Altimeter(0x52)),how='outer',on=['utc','jst'])
    # df=pd.merge(df,read_log('0608',Tachometer(0x31)),how='outer',on=['utc','jst'])
    # df=pd.merge(df,read_log('0608',Pitot(0x21)),how='outer',on=['utc','jst'])
    df=pd.merge(df,read_log('0608',Barometer(0x90)),how='outer',on=['utc','jst'])
    # df=pd.merge(df,read_log('0608',GPS(0x06)),how='outer',on=['utc','jst'])
    # df=pd.merge(df,read_log('0608',IMU(0x40)),how='outer',on=['utc','jst'])
    # df=pd.merge(df,read_log('0608',IMU(0x41)),how='outer',on=['utc','jst'])
    # df=pd.merge(df,read_log('0608',IMU(0x42)),how='outer',on=['utc','jst'])

    
    df=df.sort_values('utc')
    df=df.interpolate()
    
    df=df[(df['jst']>=start)&(df['jst']<=end)]

    p=df['pressure_90'].to_numpy()
    z=z-z[0]
    z_est=44330 * (1.0 - (p / p[0])** 0.1903)

    e_z=df['alt_52'].to_numpy()-z_est
    df['z_est']=z_est

    dz=np.diff(z)
    e=np.array([
        dz[1:],
        dz[:-1]
    ])
    print(f"{np.std(z_est)=}")
    print(f"{np.std(dz)=}")
    xcov=np.linalg.inv(np.corrcoef(e))
    d=[np.sqrt(de@xcov@de) for de in e.T]
    ax0=plt.subplot(211)
    ax0.scatter(jst,z,color='orange',label="raw data")
    ax0.plot(df['jst'],z_est,color='green',label=r'$\hat{z}$')
    ax0.plot(jst[1:],(z[1:]+z[:-1])/2,color='red',label='z')
    ax0.legend()
    ax1=plt.subplot(212)
    ax1.plot(jst[2:],d)
    ax1.axhline(y=1.65,color='red')
    plt.show()