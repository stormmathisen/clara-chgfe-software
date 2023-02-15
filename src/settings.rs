use serde::{Serialize, Deserialize};
use serde_json;
use std::io::Error;

#[derive(Serialize, Deserialize, Debug)]
pub enum CalibrationReference {
    REF500mV,
    REF1000mV,
    REF2048mV,
    REF4096mV
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Calibration {
    pub reference: CalibrationReference,
    pub level: u8,
    pub trigger: u8,
    pub offset: u16
}
#[derive(Serialize, Deserialize, Debug)]

pub struct InputOutput {
    pub input: Input,
    pub output: Output,
    pub reference: IOReference
}
#[derive(Serialize, Deserialize, Debug)]

pub enum Input {
    EXT,
    ALT,
    CAL
}
#[derive(Serialize, Deserialize, Debug)]

pub enum Output {
    LOCAL,
    TERM
}
#[derive(Serialize, Deserialize, Debug)]

pub enum IOReference {
    REF500mV,
    REF1000mV,
    REFMANUAL
}
#[derive(Serialize, Deserialize, Debug)]

pub enum Integrator {
    FB0,
    FB1,
    FB2,
    FB3,
    FB4,
    FB5
}
#[derive(Serialize, Deserialize, Debug)]

pub struct Power {
    pub positive: bool,
    pub negative: bool,
    pub integrator: bool
}
#[derive(Serialize, Deserialize, Debug)]

pub struct Metadata {
    pub last_changed: [i128; 2],
    pub device_name: String,
    pub device_location: String
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub calibration: Calibration,
    pub io: InputOutput,
    pub integrator: Integrator,
    pub power: Power,
    pub meta: Metadata
}

impl Settings {
    pub fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        let mut bytes:Vec<u8> = Vec::new();

        //Convert calibration
        let mut byte: u8 = 0;
        match self.calibration.reference {
            CalibrationReference::REF500mV => {byte = byte | 0x01},
            CalibrationReference::REF1000mV => {byte = byte | 0x02},
            CalibrationReference::REF2048mV => {byte = byte | 0x04},
            CalibrationReference::REF4096mV => {byte = byte | 0x08},
        }
        bytes.push(byte);
        byte = 0;
        byte = self.calibration.trigger;
        bytes.push(byte);
        byte = 0;
        byte = ((self.calibration.offset & 0xFF00) >> 8) as u8;
        bytes.push(byte);
        byte = 0;
        byte = (self.calibration.offset & 0x00FF) as u8;
        bytes.push(byte);
        //Convert IO
        byte = 0;
        match self.io.input {
            Input::EXT => byte = byte | 0x01,
            Input::ALT => byte = byte | 0x02,
            Input::CAL => byte = byte | 0x04,
        }
        match self.io.output {
            Output::LOCAL => byte = byte | 0x08,
            Output::TERM => byte = byte | 0x10,
        }
        match self.io.reference {
            IOReference::REF500mV => byte = byte | 0x20,
            IOReference::REF1000mV => byte = byte | 0x40,
            IOReference::REFMANUAL => byte = byte | 0x80,
        }
        bytes.push(byte);
        byte = 0;
        //Convert Integrator
        match self.integrator {
            Integrator::FB0 => byte = byte | 0x01,
            Integrator::FB1 => byte = byte | 0x02,
            Integrator::FB2 => byte = byte | 0x04,
            Integrator::FB3 => byte = byte | 0x08,
            Integrator::FB4 => byte = byte | 0x10,
            Integrator::FB5 => byte = byte | 0x20
        }
        bytes.push(byte);
        byte = 0;
        //Convert Power
        match self.power.positive {
            true => byte = byte | 0x01,
            false => (),
        }
        match self.power.negative {
            true => byte = byte | 0x02,
            false => (),
        }
        match self.power.integrator {
            true => byte = byte | 0x04,
            false => (),
        }
        bytes.push(byte);
        Ok(bytes)
    }
    pub fn hack(&mut self) -> Result<(), Error> {
        let now = std::time::SystemTime::now();
        let ts = now.duration_since(std::time::UNIX_EPOCH).unwrap();
        let secs = ts.as_secs() as i128;
        let nanos = ts.as_nanos() - (secs * 1e9 as i128) as u128;
        self.meta.last_changed = [
            secs,
            nanos as i128
        ];
        Ok(())
    }
    pub fn to_json(&mut self) -> Result<String, serde_json::Error> {
        let json_str = serde_json::to_string(self);
        match json_str {
            Ok(j) => {
                Ok(j)
            }
            Err(e) => {
                return Err(e)
            }
        }
    }
    pub fn from_json(&mut self, s: &str) -> Result<(), serde_json::Error> {
        let j = serde_json::from_str(s);
        match j {
            Ok(j) => {
                *self = j;
            }
            Err(e) => {
                return Err(e)
            }
        }
        Ok(())
    }
}


impl Default for Settings {
    fn default() -> Settings {
        Settings {
            calibration: Calibration { reference: CalibrationReference::REF500mV, level: 255, trigger: 1, offset: 1 },
            io: InputOutput { input: Input::EXT, output: Output::TERM, reference: IOReference::REF500mV },
            integrator: Integrator::FB5,
            power: Power { positive: true, negative: true, integrator: true },
            meta: Metadata { last_changed: [-1, -1], device_name: "".to_string(), device_location: "".to_string() }
        }
    }
}


/*TODO: Implement following functions:
Convert to send_bytes
Update timestamp
Iterators
*/