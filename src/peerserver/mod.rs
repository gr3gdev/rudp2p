use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::net::{IpAddr, SocketAddr, UdpSocket};
use std::sync::{Arc, Mutex};
use std::{io, thread};

use crate::logger::Logger;
use crate::{PeerError, PeerServer, PeerSocket};

static END: &[u8] = "PL3AZE 5T0P".as_bytes();

impl PeerServer {
    pub(crate) fn new(port: u16) -> Result<PeerServer, PeerError> {
        "127.0.0.1"
            .parse::<IpAddr>()
            .or_else(|e| Err(PeerError::new_addr("Error when parse local address", e)))
            .and_then(|addr| {
                let socket_addr = SocketAddr::new(addr, port);
                UdpSocket::bind(socket_addr)
                    .or_else(|e| Err(PeerError::new_io("Error when binding socket", e)))
                    .and_then(|socket| {
                        socket.set_nonblocking(true).unwrap();
                        PeerSocket::new(socket).and_then(|socket| {
                            Ok(PeerServer {
                                socket,
                                job: None,
                                peers: Arc::new(Mutex::new(HashMap::new())),
                            })
                        })
                    })
            })
    }

    pub(crate) fn start<F>(&mut self, observer: F)
    where
        F: FnMut(Vec<u8>, SocketAddr) + Send + Sync + 'static,
    {
        let mut buf = [0; 2048];
        let shared_observer = Arc::new(Mutex::new(RefCell::new(observer)));
        let socket = self.socket.socket.try_clone().unwrap();
        let job = thread::spawn(move || {
            loop {
                let guard = shared_observer.lock().unwrap();
                match socket.recv_from(&mut buf) {
                    Ok((number_of_bytes, addr)) => {
                        let data = buf[..number_of_bytes].to_vec();
                        if data == END {
                            Logger::info("Peer server stopped.");
                            break;
                        }
                        let ref mut obs = *guard.borrow_mut();
                        obs(data, addr);
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                        // wait until network socket is ready
                    }
                    Err(e) => Logger::error(e),
                }
            }
        });
        Logger::info("Peer server started.");
        self.job = Some(job);
    }

    pub(crate) fn is_alive(&self) -> bool {
        self.job.is_none() || !self.job.as_ref().unwrap().is_finished()
    }

    pub(crate) fn stop(&self) -> Result<(), PeerError> {
        let socket = &self.socket;
        socket
            .addr()
            .and_then(|address| socket.send_buffer_to(END, address))
            .and_then(|_| Ok(()))
    }
}

impl Debug for PeerServer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PeerServer")
            .field("socket", &self.socket)
            .finish()
    }
}
