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
    df=read_log('0628',IMU(0x40))
    print(len(df)/(df['jst'].iloc[-1]-df['jst'].iloc[0]).total_seconds())

    plt.scatter(df['jst'],df['timestamp_40'])
    plt.show()