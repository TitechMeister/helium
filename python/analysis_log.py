from read_txt import read_log
import pandas as pd
from datetime import datetime,timezone,timedelta

if __name__=='__main__':
    from sensor import ServoController,Vane
    servo=ServoController()
    servo_df=read_log('0608',16,servo)

    vane=Vane()
    vane_df=read_log('0608',113,vane)

    # 2024/06/08 4:51 から3分間で取得したデータを切り出す。
    JST = timezone(timedelta(hours=+9))
    start=datetime(2024,6,8,6,54,00,tzinfo=JST)
    end=  start + timedelta(seconds=180)
    servo_df_5th=servo_df[(servo_df['jst']>start)&(servo_df['jst']<end)]
    vane_df_5th=vane_df[(vane_df['jst']>start)&(vane_df['jst']<end)]

    from matplotlib import pyplot as plt,gridspec

    fig=plt.figure()
    gs=gridspec.GridSpec(3,1)
    ax11 = plt.subplot(gs[:2])
    ax11.set_ylim(-20,20)
    ax12= ax11.twinx()
    ax2 = plt.subplot(gs[2:],sharex=ax11)

    ax11.plot(servo_df_5th['jst'],servo_df_5th['rudder'],color='red',label="rudder")
    ax12.plot(servo_df_5th['jst'],servo_df_5th['elevator'],color='blue',label="elevator")
    plt.setp(ax11.get_xticklabels(), visible=False)
    ax11.set_ylabel('rudder (deg)',color='red')
    ax12.set_ylabel('elevator (deg)',color='blue')
    ax11.tick_params('y',colors='red')
    ax12.tick_params('y',colors='blue')

    ax2.plot(vane_df_5th['jst'],vane_df_5th['angle'],label=r'$\beta$')
    ax2.set_ylabel(r'$\beta$ (deg)')
    ax2.legend()

    fig.subplots_adjust(hspace=0.0)
    plt.show()