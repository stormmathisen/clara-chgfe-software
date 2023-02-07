use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc::{sync_channel, SyncSender, Receiver, TrySendError, TryRecvError},
};
use anyhow::{Result, Error, Context};
use std::io::{prelude::*, BufReader};
use std::time::Duration;


fn handle_stream(mut s: TcpStream, c: SyncSender<String>) {
    s.set_read_timeout(Some(Duration::from_secs(60))).unwrap();
    let mut buf_reader = BufReader::new(&mut s);
    loop {
        let mut line = String::new();
        let result = buf_reader.read_line(&mut line);
        match result {
            Ok(u) if u > 0 => {
                let result = c.try_send(line.trim_end_matches("\n").to_string());
                match result {
                    Ok(_) => {

                    }
                    Err(e) if e == TrySendError::Disconnected("".to_string()) => {
                        println!("Channel disconnected");
                        break;
                    }
                    Err(e) if e == TrySendError::Full("".to_string()) => {
                        println!("Channel full!");
                    }
                    Err(_) => {
                        println!("Unknown error!")
                    }
                }
            },
            Ok(u) => {
                println!("Connection dropped");
                break;
            }
            Err(e) => {
                println!("{:?}", e.kind());
                println!("Dropping connection");
                break;
            },
        }
    }
}

pub fn tcp_listener(control_channel: Receiver<bool>, data_channel: SyncSender<String>) -> Result<(), Error> {
    let listener = TcpListener::bind("0.0.0.0:56000")?;
    listener.set_nonblocking(true)?;
    println!("Starting listening");
    for stream in listener.incoming() {
        match stream {
            Ok(mut s) => {
                println!("Handling stream");
                handle_stream(s, data_channel.clone());
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