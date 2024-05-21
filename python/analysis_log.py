from cobs import cobs_decode
from sensor import Altimeter,IMU,Pitot,ServoController,Tachometer
from tqdm import tqdm
import numpy as np
from matplotlib import pyplot as plt
from mpl_toolkits.mplot3d import Axes3D
from scipy.spatial.transform import Rotation as R
from matplotlib.animation import FuncAnimation


def drawAircraft(quat:np.ndarray):
    SCALE=5
    from matplotlib import pyplot as plt
    vertices=np.array([
        [2,0,0],[0,2,-0.5],[-1.5,2,-0.5],[-1,0,0],[-1.5,-2,-0.5],[0,-2,-0.5],[2,0,0],
        [0 ,0,1],[-1,0,1],[-1,0,0],[2,0,0]
    ]).T
    # vertices=np.array([
    #     [1,0,0],[0,0,0]]).T
    r=R.from_quat(quat).as_matrix().T
    vertices=((r@vertices).T)
    plt.plot(vertices[:,0],vertices[:,1],vertices[:,2])

if __name__=="__main__":
    with open("../log.bin","rb") as f:
        data = f.read()
    servo_controller = ServoController()
    altimeter = Altimeter()
    imu40 = IMU()
    imu41 = IMU()
    imu42 = IMU()
    pitot = Pitot()
    tachometer = Tachometer()
    with tqdm(total=len(data)+1) as pbar:
        while len(data)>0:
            before = len(data)
            dec, data = cobs_decode(data)
            match dec[0] & 0xF0:
                case 0x10:
                    servo_controller.parse(dec)
                case 0x20:
                    pitot.parse(dec)
                case 0x30:
                    tachometer.parse(dec)
                case 0x40:
                    match dec[0]&0x0F:
                        case 0x00:
                            imu40.parse(dec)
                        case 0x01:
                            imu41.parse(dec)
                        case 0x02:
                            imu42.parse(dec)
                case 0x50:
                    altimeter.parse(dec)
            pbar.update(before-len(data))


    fig = plt.figure()
    ax = fig.add_subplot(111, projection='3d')
    ax.set_xlim(-10,10)
    ax.set_ylim(-10,10)
    ax.set_zlim(-10,10)
    ax.set_xlabel("X")
    ax.set_ylabel("Y")
    ax.set_zlabel("Z")
    ax.set_title("Aircraft")

    def update(frame):
        ax.cla()
        ax.set_xlim(-10,10)
        ax.set_ylim(-10,10)
        ax.set_zlim(-10,10)
        ax.set_xlabel("X")
        ax.set_ylabel("Y")
        ax.set_zlabel("Z")
        ax.set_title(f"Aircraft, frame {frame}")
        drawAircraft(imu42.raw_data["quat"][frame])
    ani = FuncAnimation(fig, update, frames=range(0,len(imu42.raw_data["timestamp"])), blit=False, interval=100)
    plt.show()
