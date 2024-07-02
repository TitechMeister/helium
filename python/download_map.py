import requests
from itertools import product
import numpy as np
import os
import cv2

def get_tile(z, x, y):
    """
    今回は標準地図を取得するとしてURLを指定する。
    与えられた座標に対応する地理院地図タイル画像を保存する。
    """
    # url = "https://cyberjapandata.gsi.go.jp/xyz/std/{}/{}/{}.png".format(z, x, y)
    url =f"https://cyberjapandata.gsi.go.jp/xyz/std/{z}/{x}/{y}.png"

    os.makedirs(f"tile/{z}", exist_ok=True)

    file_name = f"../assets/map/tile/{z}/{y}_{x}.png"

    response = requests.get(url)
    image = response.content

    with open(file_name, "wb") as f:
        f.write(image)

# x=int((2.0**(MAP_ZOOM+7.0))*(df['lon_60'].iloc[-1]/180.0+1))
# y=int((2.0**(MAP_ZOOM+7.0))/np.pi*(-np.arctanh(np.sin(np.radians(df['lat_60'].iloc[-1]))) + np.arctanh(np.sin(np.radians(85.05112878)))))
# if ((x//256)!=MAP_X) or ((y//256)!=MAP_Y):
#     MAP_X=x//256
#     MAP_Y=y//256
#     map_img=cv2.imread(f'map/{MAP_ZOOM}-{MAP_X}-{MAP_Y}.png')

# 琵琶湖周辺
# zoom_level, X, Y = 10, np.arange(899, 901+1), np.arange(403, 405+1)
# zoom_level, X, Y = 11, np.arange(1797,1799+1), np.arange(807,810+1)
# zoom_level, X, Y = 12, np.arange(3594, 3599+1), np.arange(1615, 1620+1)
# zoom_level, X, Y = 13, np.arange(7188,7197), np.arange(3230,3241+1)
zoom_level, X, Y = 14, np.arange(14377,14394+1), np.arange(6461,6483+1)

# 東工大周辺
# zoom_level, X, Y = 16, np.arange(58196,58196+1), np.arange(25823,25823+1)
# zoom_level, X, Y = 17, np.arange(116392,116393+1), np.arange(51646,51649+1)
# zoom_level, X, Y = 18, np.arange(232785,232787+1), np.arange(103293,103299+1)

# for (i,j) in product(X,Y):
#     get_tile(zoom_level, i, j)

def get_tile_area(min, max):
    """
    Get tiles in the area specified by min and max.
    """
    assert min[0] == max[0], "check zoom level"
    zoom = min[0]
    im_v_lst = []
    for i in range(max[1]-min[1]+1):
        for j in range(max[2]-min[2]+1):
            filepath = path = "tile/{}_{}_{}.jpg".format(zoom, i+min[1], j+min[2])
            if os.path.exists(filepath) == True:
                continue
            get_tile(zoom, i+min[1], j+min[2])

def cat_tile(north_west, south_east):
    zoom = north_west[0]
    im_v_lst = []
    for i in range(south_east[2]-north_west[2]+1):
        im_h_lst = []
        for j in range(south_east[1]-north_west[1]+1):
            path = "tile/{}_{}_{}.jpg".format(zoom, j+north_west[1], i+north_west[2])
            im1 = cv2.imread(path,-1)
            im_h_lst.append(im1)
        im_h = cv2.hconcat(im_h_lst)
        im_v_lst.append(im_h)
    im_v = cv2.vconcat(im_v_lst)
    cv2.imwrite("tile/tile.png", im_v)


# get_tile_area((14, 14377, 6461), (14, 14394, 6483))

cat_tile((14, 14377, 6461), (14, 14394, 6483))

# get_tile(16, 58196, 25823)