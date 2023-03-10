pub mod uart;
pub mod tcpip;
pub mod settings;
pub mod debug_terminal;

use std::io::{Error};
use std::time::{Duration};
use std::thread::sleep;
use text_io::read;

const UART_PATH: &str = "/dev/serial0";

fn main() -> Result<(), Error> {
    println!("Hello, world!");
    let mut settings = settings::Settings {..Default::default()};
    settings.hack().unwrap();
    println!("{:?}", settings.meta.last_changed);
    let test_byte:u8 = 0x80;
    let mut test_vector:Vec<u8> = vec![0,1,2,3,4,5];

    let mut fd = uart::setup_uart(UART_PATH, std::time::Duration::from_millis(100), 115200)?;

    loop {
        let input: String = read!();
        
        match input.to_lowercase().as_str() {
            "exit" => {break;},
            _ => {debug_terminal::decode(input,  &mut settings).unwrap();}
        }

        println!("{:?}", settings);

        let bytes = settings.to_bytes()?;

        println!("Broadcasting: {:?}", bytes);
        uart::send_bytes(&mut fd, &bytes)?;
    }

    Ok(())

}

//Use Querystrings for commands? field and value, in vector form
/*
TODO:
Finish settings
Test with UART only
Implement tcpip with querystrings for incoming commands
Write some helper binaries for setting functions
 */