use std::string;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum CalibrationReference {
    REF500mV,
    REF1000mV,
    REF2048mV,
    REF4096mV
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Calibration {
    reference: CalibrationReference,
    level: u8,
    trigger: u8,
    offset: u16
}
#[derive(Serialize, Deserialize, Debug)]

pub struct InputOutput {
    input: Input,
    output: Output,
    reference: IOReference
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
    FB1,
    FB2,
    FB3,
    FB4,
    FB5
}
#[derive(Serialize, Deserialize, Debug)]

pub struct Power {
    positive: bool,
    negative: bool,
    integrator: bool
}
#[derive(Serialize, Deserialize, Debug)]

pub struct Metadata {
    last_changed: [i128; 2],
    device_name: String,
    device_location: String
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub calibration: Calibration,
    pub io: InputOutput,
    pub integrator: Integrator,
    pub power: Power,
    pub meta: Metadata
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