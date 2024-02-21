import json
from enum import Enum


class Settings:
    def __init__(self):
        calibration = {
            "reference": "REF500mV",
            "level": 128,
            "trigger": 1,
            "offset": 1
        }
        inputoutput = {
            "input": "EXT",
            "output": "TERM",
            "reference": "REF500mV"
        }
        power = {
            "positive": True,
            "negative": True,
            "integrator": True
        }
        meta = {
            "last_changed": [0, 0],
            "device_name": "",
            "device_location": ""
        }
        self.settings = {
            "calibration": calibration,
            "io": inputoutput,
            "integrator": "FB0",
            "power": power,
            "meta": meta
        }

    def to_json(self):
        return json.dumps(self.settings, separators=(',', ':'))
    
    def from_json(self, j):
        try:
            self.settings = json.loads(j)
            return True
        except:
            return False

    def set_calibration_reference(self, ref):
        if ref == 'REF500mV':
            self.settings['calibration']['reference'] = "REF500mV"
            return True
        elif ref == 'REF1000mV':
            self.settings['calibration']['reference'] = "REF1000mV"
            return True
        elif ref == 'REF2048mV':
            self.settings['calibration']['reference'] = "REF2048mV"
            return True
        elif ref == 'REF4096mV':
            self.settings['calibration']['reference'] = "REF4096mV"
            return True
        else:
            return False

    def set_calibration_level(self, level):
        if level >= 0 and level < 256:
            self.settings['calibration']['level'] = level
            return True
        else:
            return False
    
    def set_calibration_trigger(self, trigger):
        if trigger >= 0 and trigger < 256:
            self.settings['calibration']['trigger'] = trigger
            return True
        else:
            return False


    def set_calibration_offset(self, offset):
        if offset >= 0 and offset < 65536:
            self.settings['calibration']['offset'] = offset
            return True
        else:
            return False

    def set_io_input(self, input):
        if input == 'EXT':
            self.settings['io']['input'] = "EXT"
            return True
        elif input == 'ALT':
            self.settings['io']['input'] = "ALT"
            return True
        elif input == 'CAL':
            self.settings['io']['input'] = "CAL"
            return True
        else:
            return False

    def set_io_output(self, output):
        if output == 'TERM':
            self.settings['io']['output'] = "TERM"
            return True
        elif output == 'LOCAL':
            self.settings['io']['output'] = "LOCAL"
            return True
        else:
            return False

    def set_io_reference(self, reference):
        if reference == 'REF500mV':
            self.settings['io']['reference'] = "REF500mV"
            return True
        elif reference == 'REF1000mV':
            self.settings['io']['reference'] = "REF1000mV"
            return True
        elif reference == 'REFMANUAL':
            self.settings['io']['reference'] = "REFMANUAL"
            return True
        else:
            return False

    def set_integrator(self, integrator):
        if integrator == 'FB0':
            self.settings['integrator'] = "FB0"
            return True
        elif integrator == 'FB1':
            self.settings['integrator'] = "FB1"
            return True
        elif integrator == 'FB2':
            self.settings['integrator'] = "FB2"
            return True
        elif integrator == 'FB3':
            self.settings['integrator'] = "FB3"
            return True
        elif integrator == 'FB4':
            self.settings['integrator'] = "FB4"
            return True
        elif integrator == 'FB5':
            self.settings['integrator'] = "FB5"
            return True
        else:
            return False

    def set_power_positive(self, power):
        if power:
            self.settings['power']['positive'] = True
            return True
        else:
            self.settings['power']['positive'] = False
            return True

    def set_power_negative(self, power):
        if power:
            self.settings['power']['negative'] = True
            return True
        else:
            self.settings['power']['negative'] = False
            return True

    def set_power_integrator(self, power):
        if power:
            self.settings['power']['integrator'] = True
            return True
        else:
            self.settings['power']['integrator'] = False
            return True

if __name__ == "__main__":
    import socket
    settings = Settings()
    settings.settings["calibration"]["reference"] = "REF1000mV"
    print(settings.to_json())
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.connect(("192.168.93.13", 56000))
        s.sendall(bytes(settings.to_json()+"\n", "utf-8"))
        recv = s.recv(4096)
        try:
            print(json.loads(recv.decode("utf-8")))
        except json.decoder.JSONDecodeError:
            print(recv.decode("utf-8"))
        #print(settings.settings)