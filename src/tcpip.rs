use std::net::{TcpListener, SocketAddr};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc::{sync_channel, Receiver, TrySendError, TryRecvError, RecvTimeoutError},
};

fn tcp_listener() {

}