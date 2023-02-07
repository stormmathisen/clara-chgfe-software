use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc::{sync_channel, SyncSender, Receiver, TryRecvError},
};
use anyhow::{Result, Error, Context};


fn handle_stream(mut s: TcpStream, settings: &mut super::settings::Settings) {
    let mut buffer:String = String::new();
    let result = s.read_to_string(&mut buffer);
    match result {
        Ok(u) => {
            println!("{:?}", buffer);
            super::debug_terminal::decode(buffer, settings).unwrap();
        },
        Err(_) => todo!(),
    }
}

pub fn tcp_listener(control_channel: Receiver<bool>, mut settings: &mut super::settings::Settings) -> Result<(), Error> {
    let listener = TcpListener::bind("0.0.0.0:56000")?;
    listener.set_nonblocking(true)?;
    println!("Starting listening");
    for stream in listener.incoming() {
        match stream {
            Ok(mut s) => {
                println!("Handling stream");
                handle_stream(s, &mut settings);
            },
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                
            }
            Err(e) => panic!("IO error while reading stream, {e}")
        }
        match control_channel.try_recv() {
            Ok(_) => {
                break;
            },
            Err(e) if e == TryRecvError::Empty => {
                
            },
            Err(e) => {
                break;
            }
        }
    }
    println!("Breaking tcpip thread!");
    Ok(())
}