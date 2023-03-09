use std::{io, thread};
use std::net::{IpAddr, SocketAddr, UdpSocket};
use std::thread::JoinHandle;

use crate::utils::{get_optional_shared, new_optional_closure, OptionalClosure, set_optional_closure};

pub static END: &[u8] = "STOPPING SERVER PLEASE".as_bytes();

pub trait ServerStatus {
    fn alive(&self) -> bool;
    fn addr(&self) -> SocketAddr;
}

pub trait Server<T> {
    fn new(port: u16) -> T;
    fn start(&mut self) -> ();
    fn close(&self) -> ();
    fn send(&self, msg: &[u8], addr: &SocketAddr) -> ();
}

pub struct Event {
    pub content: Vec<u8>,
    pub sender: SocketAddr,
}

impl Clone for Event {
    fn clone(&self) -> Self {
        Event {
            content: self.content.clone(),
            sender: self.sender.clone(),
        }
    }
}

pub struct Udp {
    socket: UdpSocket,
    on_started: OptionalClosure<dyn FnMut(&SocketAddr) + Send + Sync>,
    on_stopped: OptionalClosure<dyn FnMut(&SocketAddr) + Send + Sync>,
    observer: OptionalClosure<dyn FnMut(&Event, &UdpSocket) + Send + Sync>,
    job: Option<JoinHandle<()>>,
}

impl Server<Udp> for Udp {
    fn new(port: u16) -> Udp {
        let addr = "127.0.0.1".parse::<IpAddr>()
            .expect("Error on IP");
        let socket_addr = SocketAddr::new(addr, port);
        let socket = UdpSocket::bind(socket_addr).unwrap();
        socket.set_nonblocking(true).unwrap();
        Udp {
            socket,
            on_started: new_optional_closure(None),
            on_stopped: new_optional_closure(None),
            observer: new_optional_closure(None),
            job: None,
        }
    }

    fn start(&mut self) -> () {
        let socket = self.socket.try_clone().unwrap();
        let socket_addr = socket.local_addr().unwrap();
        let shared_observer = get_optional_shared(&self.observer);
        let shared_started = get_optional_shared(&self.on_started);
        let shared_stopped = get_optional_shared(&self.on_stopped);
        let mut buf = [0; 1024];
        // Démarrage du thread pour la réception des données
        let job = thread::spawn(move || {
            if let Some(ref mut on_started) = *shared_started.lock().unwrap().borrow_mut() {
                on_started(&socket_addr);
            }
            loop {
                let guard = shared_observer.lock().unwrap();
                match socket.recv_from(&mut buf) {
                    Ok((number_of_bytes, addr)) => {
                        let data = buf[..number_of_bytes].to_vec();
                        if data == END {
                            break;
                        }
                        if let Some(ref mut observer) = *guard.borrow_mut() {
                            observer(&Event {
                                content: data,
                                sender: addr,
                            }, &socket);
                        }
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                        // wait until network socket is ready
                    }
                    Err(e) => eprintln!("encountered IO error: {e}"),
                }
            }
            if let Some(ref mut on_stopped) = *shared_stopped.lock().unwrap().borrow_mut() {
                on_stopped(&socket_addr);
            }
        });
        self.job = Some(job);
    }

    fn close(&self) -> () {
        let address = self.socket.local_addr().unwrap();
        self.send(END, &address);
    }

    fn send(&self, msg: &[u8], addr: &SocketAddr) -> () {
        self.socket.send_to(msg, addr).unwrap();
    }
}

impl ServerStatus for Udp {
    fn alive(&self) -> bool {
        if let Some(job) = &self.job {
            !job.is_finished()
        } else {
            false
        }
    }

    fn addr(&self) -> SocketAddr {
        self.socket.local_addr().unwrap()
    }
}

impl Udp {
    pub fn set_observer<F>(&mut self, observer: F) where F: FnMut(&Event, &UdpSocket) + Send + Sync + 'static {
        set_optional_closure(&self.observer, Box::new(observer));
    }

    pub fn set_on_started<F>(&mut self, on_started: F) where F: FnMut(&SocketAddr) + Send + Sync + 'static {
        set_optional_closure(&self.on_started, Box::new(on_started));
    }

    pub fn set_on_stopped<F>(&mut self, on_stopped: F) where F: FnMut(&SocketAddr) + Send + Sync + 'static {
        set_optional_closure(&self.on_stopped, Box::new(on_stopped));
    }
}
