use std::string;

enum CalibrationReference {
    REF500mV,
    REF1000mV,
    REF2048mV,
    REF4096mV
}
struct Calibration {
    reference: CalibrationReference,
    level: u8,
    trigger: u8,
    offset: u16
}

struct InputOutput {
    input: Input,
    output: Output,
    reference: IOReference
}

enum Input {
    EXT,
    ALT,
    CAL
}

enum Output {
    LOCAL,
    TERM
}

enum IOReference {
    REF500mV,
    REF1000mV,
    REFMANUAL
}

enum Integrator {
    FB1,
    FB2,
    FB3,
    FB4,
    FB5
}

struct Power {
    positive: bool,
    negative: bool,
    integrator: bool
}

struct Metadata {
    last_changed: [i128; 2],
    device_name: std::string::String,
    device_location: std::string::String
}

struct Settings {
    calibration: Calibration,
    io: InputOutput,
    integrator: Integrator,
    power: Power,
    meta: Metadata
}

//TODO: Add new function, add serialize derivations