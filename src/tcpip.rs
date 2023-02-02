use std::net::{TcpListener, TcpStream};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc::{sync_channel, SyncSender, Receiver, TryRecvError},
};
use anyhow::{Result, Error};

fn handle_stream(s: TcpStream, data_channel: &mut SyncSender<String>) {

}

fn tcp_listener(mut data_channel: SyncSender<String>, control_channel: Receiver<bool>) -> Result<(), Error> {
    let listener = TcpListener::bind("0.0.0.0:56000")?;
    listener.set_nonblocking(true)?;
    
    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                handle_stream(s, &mut data_channel);
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

    Ok(())
}