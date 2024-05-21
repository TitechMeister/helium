"""
紙飛行機の姿勢制御

初期位置からオイラー角で指定した角度へクオータニオンで回転まで

参考URL
回転行列、クォータニオン(四元数)、オイラー角の相互変換
 https://qiita.com/aa_debdeb/items/3d02e28fb9ebfa357eaf

numpy-quaternionライブラリを使用
 pip install numpy-quaternion

"""

import ctypes
from enum import Enum

from urllib.parse import SplitResult
import numpy as np
import quaternion
import matplotlib.pyplot as plt
from mpl_toolkits.mplot3d import Axes3D
import mpl_toolkits.mplot3d.art3d as art3d


# オイラー角の回転順
class EulerOrder(Enum):
    XYZ=0
    XZY=1
    YXZ=2
    YZX=3
    ZXY=4
    ZYX=5

# 紙飛行機のmodelデータを作成
def plane(offset):
    x = [1,-1,-1, 1,-1,-1, 1]
    y = [0, 1,-1, 0, 0, 0, 0]
    z = [0, 0, 0, 0,-0.5, 0, 0]

    mx = list(map(lambda a: a + offset[0], x))
    my = list(map(lambda b: b + offset[1], y))
    mz = list(map(lambda c: c + offset[2], z))

    return mx, my, mz


# オイラー角で回転
# p[3]=回転前座標,
# th[3]=オイラー角（ラジアンXYZ）,
# order=回転順
# return x,y,z 回転後座標
def EulerAngles(p, th, order):
    if order == EulerOrder.XYZ:
        #XYZ
        x = ((np.cos(th[1])*np.cos(th[2]))*p[0]) + ((-np.cos(th[1])*np.sin(th[2]))*p[1]) + (np.sin(th[1])*p[2])
        y = ((np.sin(th[0])*np.sin(th[1])*np.cos(th[2])+np.cos(th[0])*np.sin(th[2]))*p[0]) + ((-np.sin(th[0])*np.sin(th[1])*np.sin(th[2])+np.cos(th[0])*np.cos(th[2]))*p[1]) + ((-np.sin(th[0])*np.cos(th[1]))*p[2])
        z = ((-np.cos(th[0])*np.sin(th[1])*np.cos(th[2])+np.sin(th[0])*np.sin(th[2]))*p[0]) + ((np.cos(th[0])*np.sin(th[1])*np.sin(th[2])+np.sin(th[0])*np.cos(th[2]))*p[1]) + ((np.cos(th[0])*np.cos(th[1]))*p[2])
    elif order == EulerOrder.XZY:
        #XZY
        x = ((np.cos(th[1])*np.cos(th[2]))*p[0]) + (-np.sin(th[2])*p[1]) + ((np.sin(th[1])*np.cos(th[2]))*p[2])
        y = ((np.cos(th[0])*np.cos(th[1])*np.sin(th[2])+np.sin(th[0])*np.sin(th[1]))*p[0]) + ((np.cos(th[0])*np.cos(th[2]))*p[1]) + ((np.cos(th[0])*np.sin(th[1])*np.sin(th[2])-np.sin(th[0])*np.cos(th[1]))*p[2])
        z = ((np.sin(th[0])*np.cos(th[1])*np.sin(th[2])-np.cos(th[0])*np.sin(th[1]))*p[0]) + ((np.sin(th[0])*np.cos(th[2]))*p[1]) + ((np.sin(th[0])*np.sin(th[1])*np.sin(th[2])+np.cos(th[0])*np.cos(th[1]))*p[2])
    elif order == EulerOrder.YXZ:
        #YXZ
        x = ((np.sin(th[0])*np.sin(th[1])*np.sin(th[2])+np.cos(th[1])*np.cos(th[2]))*p[0]) + ((np.sin(th[0])*np.sin(th[1])*np.cos(th[2])-np.cos(th[1])*np.sin(th[2]))*p[1]) + ((np.cos(th[0])*np.sin(th[1]))*p[2])
        y = ((np.cos(th[0])*np.sin(th[2]))*p[0]) + ((np.cos(th[0])*np.cos(th[2]))*p[1]) + ((-np.sin(th[0]))*p[2])
        z = ((np.sin(th[0])*np.cos(th[1])*np.sin(th[2])-np.sin(th[1])*np.cos(th[2]))*p[0]) + ((np.sin(th[0])*np.cos(th[1])*np.cos(th[2])+np.sin(th[1])*np.sin(th[2]))*p[1]) + ((np.cos(th[0])*np.cos(th[1]))*p[2])
    elif order == EulerOrder.YZX:
        #YZX
        x = ((np.cos(th[1])*np.cos(th[2]))*p[0]) + ((-np.cos(th[0])*np.cos(th[1])*np.sin(th[2])+np.sin(th[0])*np.sin(th[1]))*p[1]) + ((np.sin(th[0])*np.cos(th[1])*np.sin(th[2])+np.cos(th[0])*np.sin(th[1]))*p[2])
        y = ((np.sin(th[2]))*p[0]) + ((np.cos(th[0])*np.cos(th[2]))*p[1]) + ((-np.sin(th[0])*np.cos(th[2]))*p[2])
        z = ((-np.sin(th[1])*np.cos(th[2]))*p[0]) + ((np.cos(th[0])*np.sin(th[1])*np.sin(th[2])+np.sin(th[0])*np.cos(th[1]))*p[1]) + ((-np.sin(th[0])*np.sin(th[1])*np.sin(th[2])+np.cos(th[0])*np.cos(th[1]))*p[2])
    elif order == EulerOrder.XYZ.ZXY:
        #ZXY
        x = ((-np.sin(th[0])*np.sin(th[1])*np.sin(th[2])+np.cos(th[1])*np.cos(th[2]))*p[0]) + ((-np.cos(th[0])*np.sin(th[2]))*p[1]) + ((np.sin(th[0])*np.cos(th[1])*np.sin(th[2])+np.sin(th[1])*np.cos(th[2]))*p[2])
        y = ((np.sin(th[0])*np.sin(th[1])*np.cos(th[2])+np.cos(th[1])*np.sin(th[2]))*p[0]) + ((np.cos(th[0])*np.cos(th[2]))*p[1]) + ((-np.sin(th[0])*np.cos(th[1])*np.cos(th[2])+np.sin(th[1])*np.sin(th[2]))*p[2])
        z = ((-np.cos(th[0])*np.sin(th[1]))*p[0]) + ((np.sin(th[0]))*p[1]) + ((np.cos(th[0])*np.cos(th[1]))*p[2])
    elif order == EulerOrder.ZYX:
        #ZYX
        x = ((np.cos(th[1])*np.cos(th[2]))*p[0]) + ((np.sin(th[0])*np.sin(th[1])*np.cos(th[2])-np.cos(th[0])*np.sin(th[2]))*p[1]) + ((np.cos(th[0])*np.sin(th[1])*np.cos(th[2])+np.sin(th[0])*np.sin(th[2]))*p[2])
        y = ((np.cos(th[1])*np.sin(th[2]))*p[0]) + ((np.sin(th[0])*np.sin(th[1])*np.sin(th[2])+np.cos(th[0])*np.cos(th[2]))*p[1]) + ((np.cos(th[0])*np.sin(th[1])*np.sin(th[2])-np.sin(th[0])*np.cos(th[2]))*p[2])
        z = ((-np.sin(th[1]))*p[0]) + ((np.sin(th[0])*np.cos(th[1]))*p[1]) + ((np.cos(th[0])*np.cos(th[1]))*p[2])

    return x,y,z

# オイラー角をQuaternionに変換
# th[3]=オイラー角（ラジアンXYZ）,
# order=回転順
# return q クオータニオン
def Euler2Quaternion(th, order):
    if order == EulerOrder.XYZ:
        x = np.cos(th[0]/2.0)*np.sin(th[1]/2.0)*np.sin(th[2]/2.0) + np.sin(th[0]/2.0)*np.cos(th[1]/2.0)*np.cos(th[2]/2.0)
        y = -np.sin(th[0]/2.0)*np.cos(th[1]/2.0)*np.sin(th[2]/2.0) + np.cos(th[0]/2.0)*np.sin(th[1]/2.0)*np.cos(th[2]/2.0)
        z = np.cos(th[0]/2.0)*np.cos(th[1]/2.0)*np.sin(th[2]/2.0) + np.sin(th[0]/2.0)*np.sin(th[1]/2.0)*np.cos(th[2]/2.0)
        w = -np.sin(th[0]/2.0)*np.sin(th[1]/2.0)*np.sin(th[2]/2.0) + np.cos(th[0]/2.0)*np.cos(th[1]/2.0)*np.cos(th[2]/2.0)
    elif order == EulerOrder.XZY:
        x = -np.cos(th[0]/2.0)*np.sin(th[1]/2.0)*np.sin(th[2]/2.0) + np.sin(th[0]/2.0)*np.cos(th[1]/2.0)*np.cos(th[2]/2.0)
        y = np.cos(th[0]/2.0)*np.sin(th[1]/2.0)*np.cos(th[2]/2.0) - np.sin(th[0]/2.0)*np.cos(th[1]/2.0)*np.sin(th[2]/2.0)
        z = np.sin(th[0]/2.0)*np.sin(th[1]/2.0)*np.cos(th[2]/2.0) + np.cos(th[0]/2.0)*np.cos(th[1]/2.0)*np.sin(th[2]/2.0)
        w = np.sin(th[0]/2.0)*np.sin(th[1]/2.0)*np.sin(th[2]/2.0) + np.cos(th[0]/2.0)*np.cos(th[1]/2.0)*np.cos(th[2]/2.0)
    elif order == EulerOrder.YXZ:
        x = np.cos(th[0]/2.0)*np.sin(th[1]/2.0)*np.sin(th[2]/2.0) + np.sin(th[0]/2.0)*np.cos(th[1]/2.0)*np.cos(th[2]/2.0)
        y = -np.sin(th[0]/2.0)*np.cos(th[1]/2.0)*np.sin(th[2]/2.0) + np.cos(th[0]/2.0)*np.sin(th[1]/2.0)*np.cos(th[2]/2.0)
        z = np.cos(th[0]/2.0)*np.cos(th[1]/2.0)*np.sin(th[2]/2.0) - np.sin(th[0]/2.0)*np.sin(th[1]/2.0)*np.cos(th[2]/2.0)
        w = np.sin(th[0]/2.0)*np.sin(th[1]/2.0)*np.sin(th[2]/2.0) + np.cos(th[0]/2.0)*np.cos(th[1]/2.0)*np.cos(th[2]/2.0)
    elif order == EulerOrder.YZX:
        x = np.sin(th[0]/2.0)*np.cos(th[1]/2.0)*np.cos(th[2]/2.0) + np.cos(th[0]/2.0)*np.sin(th[1]/2.0)*np.sin(th[2]/2.0)
        y = np.sin(th[0]/2.0)*np.cos(th[1]/2.0)*np.sin(th[2]/2.0) + np.cos(th[0]/2.0)*np.sin(th[1]/2.0)*np.cos(th[2]/2.0)
        z = -np.sin(th[0]/2.0)*np.sin(th[1]/2.0)*np.cos(th[2]/2.0) + np.cos(th[0]/2.0)*np.cos(th[1]/2.0)*np.sin(th[2]/2.0)
        w = -np.sin(th[0]/2.0)*np.sin(th[1]/2.0)*np.sin(th[2]/2.0) + np.cos(th[0]/2.0)*np.cos(th[1]/2.0)*np.cos(th[2]/2.0)
    elif order == EulerOrder.ZXY:
        x = -np.cos(th[0]/2.0)*np.sin(th[1]/2.0)*np.sin(th[2]/2.0) + np.sin(th[0]/2.0)*np.cos(th[1]/2.0)*np.cos(th[2]/2.0)
        y = np.cos(th[0]/2.0)*np.sin(th[1]/2.0)*np.cos(th[2]/2.0) + np.sin(th[0]/2.0)*np.cos(th[1]/2.0)*np.sin(th[2]/2.0)
        z = np.sin(th[0]/2.0)*np.sin(th[1]/2.0)*np.cos(th[2]/2.0) + np.cos(th[0]/2.0)*np.cos(th[1]/2.0)*np.sin(th[2]/2.0)
        w = -np.sin(th[0]/2.0)*np.sin(th[1]/2.0)*np.sin(th[2]/2.0) + np.cos(th[0]/2.0)*np.cos(th[1]/2.0)*np.cos(th[2]/2.0)
    elif order == EulerOrder.ZYX:
        x = np.sin(th[0]/2.0)*np.cos(th[1]/2.0)*np.cos(th[2]/2.0) - np.cos(th[0]/2.0)*np.sin(th[1]/2.0)*np.sin(th[2]/2.0)
        y = np.sin(th[0]/2.0)*np.cos(th[1]/2.0)*np.sin(th[2]/2.0) + np.cos(th[0]/2.0)*np.sin(th[1]/2.0)*np.cos(th[2]/2.0)
        z = -np.sin(th[0]/2.0)*np.sin(th[1]/2.0)*np.cos(th[2]/2.0) + np.cos(th[0]/2.0)*np.cos(th[1]/2.0)*np.sin(th[2]/2.0)
        w = np.sin(th[0]/2.0)*np.sin(th[1]/2.0)*np.sin(th[2]/2.0) + np.cos(th[0]/2.0)*np.cos(th[1]/2.0)*np.cos(th[2]/2.0)

    q = np.quaternion(w,x,y,z)
    return q


def PaperAirplaneQuaternion(angle, order):
    fig = plt.figure(figsize=(8,6))
    ax = fig.add_subplot(111, projection='3d')
    plt.cla()

    th9 = [0.0]*3

    # 飛行機のモデル作成    
    x,y,z = plane([0,0,0])          #ベース3D表示用
    x2,y2,z2 = [0.0]*7, [0.0]*7, [0.0]*7    #移動中3D表示用     青色の飛行機
    x9,y9,z9 = [0]*7,[0]*7,[0]*7            #最終姿勢3D表示用   赤色の飛行機

    # 最終姿勢
    th9[0] = angle[0] * np.pi / 180.0
    th9[1] = angle[1] * np.pi / 180.0
    th9[2] = angle[2] * np.pi / 180.0
    
    for i in range(7):
        x9[i],y9[i],z9[i] = EulerAngles([x[i],y[i],z[i]], th9, order)
    
    # 最終姿勢のオイラー角をクォータニオンに置換
    Q9 = Euler2Quaternion(th9, order)
    T1 = np.arcsin(Q9.imag) * 2.0
    T = T1 / np.linalg.norm(T1)       # 回転用 単位ベクトル

    rotate = 0.0
    speed = 5.0
    RotateEnd = False

    # 回転を表すクォータニオン
    R = np.quaternion()
    
    while True:
        plt.cla()

        # 回転中の計算
        th2 = rotate * np.pi / 180
        R.real = np.cos(th2/2.0)
        R.imag = T * np.sin(th2/2.0)

        for i in range(7):
            Q1 = np.quaternion(1, x[i], y[i], z[i])     # 初期位置のクォータニオン
            Q2 = R * Q1 * R.conj()                      # 回転の計算
            x2[i], y2[i], z2[i] = Q2.x, Q2.y, Q2.z      # 表示用座標
        
        # 最終姿勢の近くに来たらループから抜ける
        if (R.w - Q9.w) > 0:
            if round(Q9.w * 100000) == round(R.w * 100000):
                speed = (R.w - Q9.w)
            elif round(Q9.w * 10000) == round(R.w * 10000):
                speed = 1
            elif round((Q9.w)*10) == round((R.w)*10):
                speed = ((R.w - Q9.w) * 500) / 2

        else:
            #最終姿勢の描画
            for i in range(7):
                Q1 = np.quaternion(0, x[i], y[i], z[i])
                Q2 = Q9 * Q1 * Q9.conj()
                x2[i], y2[i], z2[i] = Q2.x, Q2.y, Q2.z
            RotateEnd = True

        # ----- 以下 グラフ表示用 -----
        # 法線ベクトルをプロット
        P0 = [0,0,0]
        v = T
        ax.quiver(P0[0], P0[1], P0[2], v[0], v[1], v[2], color = "orange", length = 2, arrow_length_ratio = 0.2)

        # 目標位置（赤色飛行機）
        poly1 = list(zip(x9[:4],y9[:4],z9[:4]))
        ax.add_collection3d(art3d.Poly3DCollection([poly1], color='red', linewidths=0.3, alpha=0.02))
        poly2 = list(zip(x9[3:7],y9[3:7],z9[3:7]))
        ax.add_collection3d(art3d.Poly3DCollection([poly2], color='brown', linewidths=0.3, alpha=0.02))

        # 回転中のモデル（青色飛行機）
        poly3 = list(zip(x2[:4],y2[:4],z2[:4]))
        ax.add_collection3d(art3d.Poly3DCollection([poly3], color='blue', alpha=0.5))
        poly4 = list(zip(x2[3:7],y2[3:7],z2[3:7]))
        ax.add_collection3d(art3d.Poly3DCollection([poly4], color='midnightblue', alpha=0.5))
        #ax.scatter(x2[:7], y2[:7], z2[:7], color='blue')
        
        # グラフのエリア設定
        ax.set_xlabel("x");     ax.set_ylabel("y");     ax.set_zlabel("z")
        ax.set_xlim(-2,2);      ax.set_ylim(-2,2);      ax.set_zlim(-2,2)
        ax.set_box_aspect((1,1,1))
        ax.text(-1,-1,-2.3, 'Target Euler Angle: '+format(angle[0],'.1f')+', '+format(angle[1],'.1f')+', '+format(angle[2],'.1f'), fontsize=9)

        if RotateEnd:
            break
        
        rotate = rotate + speed
        plt.pause(0.1)

    plt.show()


angle = [80.0, 120.0, 60.0]       # 最終姿勢のオイラー角
order = EulerOrder.XYZ
PaperAirplaneQuaternion(angle, order)