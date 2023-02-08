use std::net::{TcpListener, TcpStream};

use crossbeam::channel::{Sender, TrySendError, Receiver, TryRecvError};

use anyhow::{Result, Error};
use std::io::{prelude::*, BufReader, BufWriter};
use std::time::Duration;
use std::thread;

fn send_to_main(c: &Sender<String>, s: String) -> Result<(), TrySendError<String>> {
    let result = c.try_send(s.trim_end_matches("\n").to_string());
    match result {
        Ok(_) => {
            println!("Successfully sent to main thread!");
        }
        Err(e) if e == TrySendError::Disconnected("".to_string()) => {
            println!("Channel disconnected");
            return Err(e);
        }
        Err(e) if e == TrySendError::Full("".to_string()) => {
            println!("Channel full!");
            return Err(e);
        }
        Err(e) => {
            println!("Unknown error!");
            return Err(e);
        }
    }
    Ok(())
}

fn recv_from_main(w: &mut BufWriter<&TcpStream>, d: Receiver<String>) -> Result<(), std::io::Error> {
    println!("Receiving from main!");
    match d.recv_timeout(Duration::from_secs(1)) {
        Ok(s) => {
            let message = format!("{s}\n");
            match w.write_all(message.as_bytes()) {
                Ok(_) => {
                    println!("Wrote to socket!");
                    w.flush();
                },
                Err(e) => {
                    return Err(e)
                },
            }
        }
        Err(e) => {

        }
    }
    Ok(())
}

fn handle_stream(mut s: TcpStream, c: Sender<String>, d: Receiver<String>) {
    s.set_read_timeout(Some(Duration::from_secs(60))).unwrap();
    let mut buf_reader = BufReader::new(&s);
    let mut buf_writer = BufWriter::new(&s);
    loop {
        let mut line = String::new();
        let result = buf_reader.read_line(&mut line);
        match result {
            Ok(u) if u > 0 => {
                let result = send_to_main(&c, line);
                match result {
                    Ok(_) => {
                        let result = recv_from_main(&mut buf_writer, d.clone());
                        match result {
                            Ok(_) => {
        
                            }
                            Err(e) => {
                                println!("Error when writing to socket, dropping connection");
                                break;
                            }
                        }
        
                    },
                    Err(e) => {
                        break;
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

pub fn tcp_listener(control_channel: Receiver<bool>, data_tx: Sender<String>, data_rx_2: Receiver<String>) -> Result<(), Error> {
    let listener = TcpListener::bind("0.0.0.0:56000")?;
    listener.set_nonblocking(true)?;
    println!("Starting listening");
    for stream in listener.incoming() {
        match stream {
            Ok(mut s) => {
                println!("Handling stream");
                let c = data_tx.clone();
                let d = data_rx_2.clone();
                thread::spawn(move|| {
                    handle_stream(s, c, d);
                }
            );
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