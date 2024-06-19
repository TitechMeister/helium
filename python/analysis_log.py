from read_txt import read_log
import pandas as pd
from datetime import datetime,timezone,timedelta
from matplotlib import pyplot as plt,gridspec
from PIL import Image
import numpy as np

if __name__=='__main__':
    from sensor import ServoController,Vane,Altimeter,Pitot,Tachometer,GPS,Barometer,IMU
    df=read_log('0619',0x40,IMU(0x40))
    # df=pd.merge(df,read_log('0608',0x71,Vane(0x71)),how='outer',on=['utc','jst'])
    # df=pd.merge(df,read_log('0608',0x52,Altimeter(0x52)),how='outer',on=['utc','jst'])
    # df=pd.merge(df,read_log('0608',0x21,Tachometer(0x31)),how='outer',on=['utc','jst'])
    # df=pd.merge(df,read_log('0608',0x31,Pitot(0x21)),how='outer',on=['utc','jst'])
    # df=pd.merge(df,read_log('0608',0x90,Barometer(0x90)),how='outer',on=['utc','jst'])
    # df=pd.merge(df,read_log('0608',0x06,GPS(0x06)),how='outer',on=['utc','jst'])
    # df=pd.merge(df,read_log('0608',0x40,IMU(0x40)),how='outer',on=['utc','jst'])
    # df=pd.merge(df,read_log('0608',0x41,IMU(0x41)),how='outer',on=['utc','jst'])
    # df=pd.merge(df,read_log('0608',0x42,IMU(0x42)),how='outer',on=['utc','jst'])



    # 2024/06/08 4:52.587000 から3分間で取得したデータを切り出す。
    # JST = timezone(timedelta(hours=+9))
    # start=datetime(2024,6,8,4,52,1,tzinfo=JST)-timedelta(seconds=24)
    # end=  start + timedelta(seconds=120)

    # df=df[(df['jst']>=start)&(df['jst']<=end)]
    df=df.sort_values('utc')
    df=df.interpolate()
    # df.to_csv('df.csv')

    fig=plt.figure()
    gs=gridspec.GridSpec(1,1)

    ax0=plt.subplot(gs[0])
    z,x,y=14,14541,6434
    # url=f"{https://tile.openstreetmap.org/{z}/{x}/{y}.png}" # OpenStreatMap
    # url=f"{http://cyberjapandata.gsi.go.jp/xyz/std/{z}/{x}/{y}.png}" # 国土地理院
    # im=np.asarray(Image.open(f'../assets/map/{z}-{x}-{y}.png'))
    # ax0.imshow(im,cmap='gray')
    # df['x']=int((2.0**(z+7.0))*(df['lon_06']/180.0+1))%256
    # df['y']= int((2.0**(z+7.0))/np.pi*(-np.arctanh(np.sin(np.radians(df['lat_06']))) + np.arctanh(np.sin(np.radians(85.05112878)))))%256
    ax0.plot(df['jst'],df['timestamp_40'],'o',color='red')

    # ax11 = plt.subplot(gs[1])
    # ax11.set_ylim(-20,20)
    # ax12= ax11.twinx()
    # ax2 = plt.subplot(gs[2],sharex=ax11)

    # # ax0.plot(df['jst'],df['alt_52'],color='purple',label='altitude')
    # # ax0.set_ylabel('altitude (m)')
    # # plt.setp(ax0.get_xticklabels(), visible=False)
    

    # ax11.plot(df['jst'],df['rudder_10'],label="rudder")
    # ax12.plot(df['jst'],df['elevator_10'],label="elevator")
    # ax12.plot(df['jst'],df['trim_10'],label="trim")
    # plt.setp(ax11.get_xticklabels(), visible=False)
    # ax11.set_ylabel('rudder (deg)',color='red')
    # ax12.set_ylabel('elevator (deg)',color='blue')
    # ax11.tick_params('y',colors='red')
    # ax12.tick_params('y',colors='blue')

    # ax2.plot(df['jst'],df['angle_71'],label=r'$\beta$')
    # ax2.set_ylabel(r'$\beta$ (deg)')
    # ax2.legend()
    # plt.setp(ax2.get_xticklabels(), visible=False)

    # ax3 = plt.subplot(gs[3],sharex=ax2)
    # ax3.plot(df['jst'],df['cadence_31'],label='cadence')
    # ax3.legend()
    # plt.setp(ax3.get_xticklabels(), visible=False)

    # ax4=plt.subplot(gs[4],sharex=ax3)
    # ax4.plot(df['jst'],df['pressure_90'],label='p')
    # ax4.legend()

    fig.subplots_adjust(hspace=0.0)
    plt.show()