pub mod uart;
pub mod tcpip;
pub mod settings;
pub mod debug_terminal;

use anyhow::{Result, Error};

use ctrlc;

use std::sync::{
    atomic::{AtomicBool, Ordering}
};

use crossbeam::channel::{bounded, Sender, SendError, Receiver, TryRecvError};

use std::thread;

const UART_PATH: &str = "/dev/serial0";
static DONE: AtomicBool = AtomicBool::new(false);

fn main() -> Result<(), Error> {
    ctrlc::set_handler(|| DONE.store(true, Ordering::SeqCst))?;

    println!("Hello, world!");
    let mut settings = settings::Settings {..Default::default()};
    settings.hack().unwrap();
    println!("{:?}", settings.meta.last_changed);

    let mut fd = uart::setup_uart(UART_PATH, std::time::Duration::from_millis(100), 115200)?;

    let (ctl_tx, ctl_rx) = bounded::<bool>(50);
    let (data_tx, data_rx) = bounded::<String>(50);
    let (data_tx_2, data_rx_2) = bounded::<String>(50);


    let tcp_thread = thread::spawn(move|| {
        tcpip::tcp_listener(ctl_rx, data_tx.clone(), data_rx_2).unwrap();
    });

    while !DONE.load(Ordering::Relaxed) {

        match data_rx.try_recv() {
            Ok(data) => {
                debug_terminal::decode(data,  &mut settings).unwrap();
                let js = settings.to_json()?;
                match data_tx_2.try_send(js) {
                    Ok(_) => {

                    }
                    Err(e) => {
                        println!("{:?}", e);
                    }
                }
            }
            Err(e) if e == TryRecvError::Disconnected => {
                
            }
            Err(e) => {

            }
        }

        /*let input: String = read!();

        match input.to_lowercase().as_str() {
            "exit" => {break;},
            _ => {debug_terminal::decode(input,  &mut settings).unwrap();}
        }

        println!("{:?}", settings);

        let bytes = settings.to_bytes()?;

        println!("Broadcasting: {:?}", bytes);
        uart::send_bytes(&mut fd, &bytes)?;*/

        
    }
    ctl_tx.send(true);
    tcp_thread.join();

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