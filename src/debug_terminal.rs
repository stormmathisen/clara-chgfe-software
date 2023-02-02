use serde::{Serialize, Deserialize};
use serde_qs as qs;
use serde_qs::Error;


#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct Payload {
    field: String,
    setting: String
}

pub fn decode(input: String, settings: &mut super::settings::Settings) -> Result <(), Error> {
    let payload: Payload = qs::from_str(&input)?;

    match payload.field.to_lowercase().as_str() {
        "calibration.reference" => {
            match payload.setting.to_lowercase().as_str() {
                "500mv" | "500" | "0" => {settings.calibration.reference = super::settings::CalibrationReference::REF500mV},
                "1000mv" | "1000" | "1" => {settings.calibration.reference = super::settings::CalibrationReference::REF1000mV},
                "2048mv" | "2048" | "3" => {settings.calibration.reference = super::settings::CalibrationReference::REF500mV},
                "4096mv" | "4096" | "4" => {settings.calibration.reference = super::settings::CalibrationReference::REF4096mV},
                _ => ()
            }
        },
        "calibration.level" => {
            match payload.setting.parse::<u8>() {
                Ok(n) => {
                    settings.calibration.level = n;
                },
                Err(e) => {
                    println!("Unable to match due to {:?}", e.kind())
                }
            }
        },
        "calibration.trigger" => {
            match payload.setting.parse::<u8>() {
                Ok(n) => {
                    settings.calibration.trigger = n;
                },
                Err(e) => {
                    println!("Unable to match due to {:?}", e.kind())
                }
            }
        },
        "calibration.offset" => {
            match payload.setting.parse::<u16>() {
                Ok(n) => {
                    settings.calibration.offset = n;
                },
                Err(e) => {
                    println!("Unable to match due to {:?}", e.kind())
                }
            }
        },
        "io.input" => {
            match payload.setting.to_lowercase().as_str() {
                "ext" | "0" => {
                    settings.io.input = super::settings::Input::EXT;
                },
                "alt" | "1" => {
                    settings.io.input = super::settings::Input::ALT;
                },
                "cal" | "2" => {
                    settings.io.input = super::settings::Input::CAL;
                }
                _ => ()
            }
        },
        "io.output" =>  {
            match payload.setting.to_lowercase().as_str() {
                "local" | "0" => {
                    settings.io.output = super::settings::Output::LOCAL;
                },
                "term" | "1" => {
                    settings.io.output = super::settings::Output::TERM;
                },
                _ => ()
            }
        },
        "io.reference" => {
            match payload.setting.to_lowercase().as_str() {
                "500mv" | "500" | "0" => {
                    settings.io.reference = super::settings::IOReference::REF500mV;
                },
                "1000mv" | "1000" | "1" => {
                    settings.io.reference = super::settings::IOReference::REF1000mV;
                },
                "manual" | "man" | "2" => {
                    settings.io.reference = super::settings::IOReference::REFMANUAL;
                }
                _ => ()
            }
        },
        "integrator" => {
            match payload.setting.to_lowercase().as_str() {
                "fb0" | "0" => {
                    settings.integrator = super::settings::Integrator::FB0;
                },
                "fb1" | "1" => {
                    settings.integrator = super::settings::Integrator::FB1;
                },
                "fb2" | "2" => {
                    settings.integrator = super::settings::Integrator::FB2;
                },
                "fb3" | "3" => {
                    settings.integrator = super::settings::Integrator::FB3;
                },
                "fb4" | "4" => {
                    settings.integrator = super::settings::Integrator::FB4;
                },
                "fb5" | "5" => {
                    settings.integrator = super::settings::Integrator::FB5;
                },
                _ => ()
            }
        },
        "power.positive" => {
            match payload.setting.parse::<bool>() {
                Ok(b) => {
                    settings.power.positive = b;
                },
                Err(e) => println!("Failed to match {:?}", e),
            }
        },
        "power.negative" => {
            match payload.setting.parse::<bool>() {
                Ok(b) => {
                    settings.power.negative = b;
                },
                Err(e) => println!("Failed to match {:?}", e),
            }
        },
        "power.integrator" => {
            match payload.setting.parse::<bool>() {
                Ok(b) => {
                    settings.power.integrator = b;
                },
                Err(e) => println!("Failed to match {:?}", e),
            }
        },
        _ => ()
    }

    Ok(())
}