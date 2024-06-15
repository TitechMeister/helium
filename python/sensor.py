import struct
from abc import ABCMeta, abstractmethod
import numpy as np

class Sensor(metaclass=ABCMeta):
    @abstractmethod
    def parse(self,array:list[int]):
        raise NotImplementedError
    @abstractmethod
    def database(self)->dict[str,list]:
        raise NotImplementedError

class ServoController(Sensor):
    raw_data={
            "timestamp":[],
            "rudder":[],
            "elevator":[],
            "voltage":[],
            "i_rudder":[],
            "i_elevator":[],
            "trim":[],
            "status":[]
            }
    def parse(self, array: list[int]):
        _id,timestamp,rudder,elevator,voltage,i_rudder,i_elevator,trim,status=struct.unpack(">BxxxIffffffBxxx",bytes(array))
        self.raw_data["timestamp"].append(timestamp)
        self.raw_data["rudder"].append(rudder)
        self.raw_data["elevator"].append(elevator)
        self.raw_data["voltage"].append(voltage)
        self.raw_data["i_rudder"].append(i_rudder)
        self.raw_data["i_elevator"].append(i_elevator)
        self.raw_data["trim"].append(trim)
        self.raw_data["status"].append(status)
    @property
    def database(self)->dict[str,list]:
        return self.raw_data

class Pitot(Sensor):
    raw_data={
            "timestamp":[],
            "pressure":[],
            "temperature":[],
            "velocity":[],
            }
    def parse(self, array: list[int]):
        for n in range(len(array)//20):
            _id,timestamp,pressure,temperature,velocity=struct.unpack(">BxxxIfff",bytes(array[20*n:20*(n+1)]))
            self.raw_data["timestamp"].append(timestamp)
            self.raw_data["pressure"].append(pressure)
            self.raw_data["temperature"].append(temperature)
            self.raw_data["velocity"].append(velocity)
    @property
    def database(self)->dict[str,list]:
        return self.raw_data

class Tachometer(Sensor):
    raw_data={
            "timestamp":[],
            "rpm":[],
            "pwr":[],
            }
    def parse(self, array: list[int]):
        for n in range(len(array)//16):
            _id,timestamp,rpm,pwr=struct.unpack(">BxxxIff",bytes(array[16*n:16*(n+1)]))
            self.raw_data["timestamp"].append(timestamp)
            self.raw_data["rpm"].append(rpm)
            self.raw_data["pwr"].append(pwr)
    @property
    def database(self)->dict[str,list]:
        return self.raw_data

class IMU(Sensor):
    raw_data={
            "timestamp":[],
            "quat":[]
            }
    def parse(self, array: list[int]):
        for n in range(len(array)//16):
            _id,timestamp,w,x,y,z=struct.unpack(">BxxxIhhhh",bytes(array[16*n:16*(n+1)]))
            w,x,y,z = w/16384.0,x/16384.0,y/16384.0,z/16384.0
            self.raw_data["timestamp"].append(timestamp)
            self.raw_data["quat"].append(np.array([w,x,y,z]))
    @property
    def database(self)->dict[str,list]:
        return self.raw_data

class Altimeter(Sensor):
    raw_data={
            "timestamp":[],
            "alt":[],
            }
    def parse(self, array: list[int]):
        for n in range(len(array)//12):
            _id,timestamp,alt=struct.unpack(">BxxxIf",bytes(array[12*n:12*(n+1)]))
            self.raw_data["timestamp"].append(timestamp)
            self.raw_data["alt"].append(alt/100)
    @property
    def database(self)->dict[str,list]:
        return self.raw_data

class GPS(Sensor):
    raw_data={
            "timestamp":[],
            "lat":[],
            "lon":[]
            }
    def parse(self, array: list[int]):
        for n in range(len(array)//24):
            _id,timestamp,lat,lon=struct.unpack(">BxxxIdd",bytes(array[24*n:24*(n+1)]))
            self.raw_data["timestamp"].append(timestamp)
            self.raw_data["lat"].append(lat)
            self.raw_data["lon"].append(lon)
    @property
    def database(self):
        return self.raw_data

class Vane(Sensor):
    raw_data={
            "timestamp":[],
            "angle":[]
            }
    def parse(self, array: list[int]):
        for n in range(len(array)//12):
            _id,timestamp,angle=struct.unpack(">BxxxIf",bytes(array[12*n:12*(n+1)]))
            self.raw_data["timestamp"].append(timestamp)
            self.raw_data["angle"].append(angle)
    @property
    def database(self):
        return self.raw_data

class Barometer(Sensor):
    raw_data={
            "timestamp":[],
            "pressure":[],
            "temperature":[],
            }
    def parse(self, array: list[int]):
        for n in range(len(array)//16):
            _id,timestamp,pressure,temperature=struct.unpack(">BxxxIff",bytes(array[16*n:16*(n+1)]))
            self.raw_data["timestamp"].append(timestamp)
            self.raw_data["pressure"].append(pressure)
            self.raw_data["temperature"].append(temperature)
    @property
    def database(self):
        return self.raw_data
