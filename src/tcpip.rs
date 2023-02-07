use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc::{sync_channel, SyncSender, Receiver, TryRecvError},
};
use anyhow::{Result, Error, Context};
use std::io::{prelude::*, BufReader};


fn handle_stream(mut s: TcpStream, settings: &mut super::settings::Settings) {
    let mut buf_reader = BufReader::new(&mut s);
    loop {
        let mut line = String::new();
        let result = buf_reader.read_line(&mut line);
        match result {
            Ok(u) if u > 0 => {
                println!("{:?}", line);
                super::debug_terminal::decode(line.trim_end_matches("\n").to_string(), settings).unwrap();
                println!("{:?}", settings);
            },
            Ok(u) => {
                break;
            }
            Err(_) => todo!(),
        }
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