from read_txt import read_log
import pandas as pd
from datetime import datetime,timezone,timedelta

if __name__=='__main__':
    from sensor import ServoController,Vane,Altimeter,Pitot,Tachometer,GPS,Barometer
    result=read_log('0608',16,ServoController())
    result=pd.merge(result,read_log('0608',0x71,Vane()),how='outer',on=['utc','jst'])
    result=pd.merge(result,read_log('0608',0x52,Altimeter()),how='outer',on=['utc','jst'])
    result=pd.merge(result,read_log('0608',0x21,Tachometer()),how='outer',on=['utc','jst'])
    result=pd.merge(result,read_log('0608',0x31,Pitot()),how='outer',on=['utc','jst'])
    result=pd.merge(result,read_log('0608',0x90,Barometer()),how='outer',on=['utc','jst'])
    result=pd.merge(result,read_log('0608',0x06,GPS()),how='outer',on=['utc','jst'])



    # 2024/06/08 4:52.587000 から3分間で取得したデータを切り出す。
    JST = timezone(timedelta(hours=+9))
    start=datetime(2024,6,8,5,5,52,587,tzinfo=JST)
    end=  start + timedelta(seconds=120)

    df_5th=result[(result['jst']>=start)&(result['jst']<=end)]

    from matplotlib import pyplot as plt,gridspec

    fig=plt.figure()
    gs=gridspec.GridSpec(4,1)
    ax0=plt.subplot(gs[0])

    ax11 = plt.subplot(gs[1],sharex=ax0)
    ax11.set_ylim(-20,20)
    ax12= ax11.twinx()
    ax2 = plt.subplot(gs[2],sharex=ax11)

    ax0.plot(df_5th['jst'],df_5th['alt'],color='purple',label='altitude')
    ax0.set_ylabel('altitude (m)')
    plt.setp(ax0.get_xticklabels(), visible=False)

    ax11.plot(df_5th['jst'],df_5th['rudder'],color='red',label="rudder")
    ax12.plot(df_5th['jst'],df_5th['elevator'],color='blue',label="elevator")
    plt.setp(ax11.get_xticklabels(), visible=False)
    ax11.set_ylabel('rudder (deg)',color='red')
    ax12.set_ylabel('elevator (deg)',color='blue')
    ax11.tick_params('y',colors='red')
    ax12.tick_params('y',colors='blue')

    ax2.plot(df_5th['jst'],df_5th['angle'],label=r'$\beta$')
    ax2.set_ylabel(r'$\beta$ (deg)')
    ax2.legend()
    plt.setp(ax2.get_xticklabels(), visible=False)

    ax3 = plt.subplot(gs[3],sharex=ax2)
    ax3.plot(df_5th['jst'],df_5th['cadence'])

    fig.subplots_adjust(hspace=0.0)
    plt.show()