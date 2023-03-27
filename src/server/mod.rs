use std::{io, thread};
use std::fmt::{Debug, Formatter};
use std::net::{IpAddr, SocketAddr, UdpSocket};
use std::thread::JoinHandle;

use crate::utils::OptionalClosure;

static END: &[u8] = "STOPPING SERVER PLEASE".as_bytes();

pub trait ServerStatus {
    /// Return `true` if Server is alive, else `false`.
    fn alive(&self) -> bool;
    /// Return the `SocketAddr` of the Server.
    fn addr(&self) -> SocketAddr;
}

pub trait Server<T> {
    /// Start the server.
    fn start(&mut self) -> ();
    /// Stop the server.
    fn close(&self) -> ();
    /// Send a `msg` to the server with address `addr`.
    fn send(&self, msg: &[u8], addr: &SocketAddr) -> ();
}

pub struct Event {
    /// Content of the event.
    pub content: Vec<u8>,
    /// Who has triggered this event.
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
    /// UdpSocket used by the Udp server.
    socket: UdpSocket,
    /// Observer : trigger when server is started.
    on_started: OptionalClosure<dyn FnMut(&SocketAddr) + Send + Sync>,
    /// Observer : trigger when server is stopped.
    on_stopped: OptionalClosure<dyn FnMut(&SocketAddr) + Send + Sync>,
    /// Observer : trigger when server has received a message.
    on_received: OptionalClosure<dyn FnMut(&Event, &UdpSocket) + Send + Sync>,
    /// Thread job.
    job: Option<JoinHandle<()>>,
}

impl Server<Udp> for Udp {
    fn start(&mut self) -> () {
        let socket = self.socket.try_clone().unwrap();
        let socket_addr = socket.local_addr().unwrap();
        let shared_observer = self.on_received.shared();
        let shared_started = self.on_started.shared();
        let shared_stopped = self.on_stopped.shared();
        let mut buf = [0; 2048];
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
        if msg.len() > 2048 {
            panic!("Error : the message is too large !")
        }
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
    pub(crate) fn new(port: u16) -> Udp {
        let addr = "127.0.0.1".parse::<IpAddr>()
            .expect("Error on IP");
        let socket_addr = SocketAddr::new(addr, port);
        let socket = UdpSocket::bind(socket_addr).unwrap();
        socket.set_nonblocking(true).unwrap();
        Udp {
            socket,
            on_started: OptionalClosure::new(None),
            on_stopped: OptionalClosure::new(None),
            on_received: OptionalClosure::new(None),
            job: None,
        }
    }

    pub fn set_on_received<F>(&mut self, observer: F) where F: FnMut(&Event, &UdpSocket) + Send + Sync + 'static {
        OptionalClosure::set(&self.on_received, Box::new(observer));
    }

    pub fn set_on_started<F>(&mut self, on_started: F) where F: FnMut(&SocketAddr) + Send + Sync + 'static {
        OptionalClosure::set(&self.on_started, Box::new(on_started));
    }

    pub fn set_on_stopped<F>(&mut self, on_stopped: F) where F: FnMut(&SocketAddr) + Send + Sync + 'static {
        OptionalClosure::set(&self.on_stopped, Box::new(on_stopped));
    }
}

impl Debug for Event {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.content)
    }
}

impl Debug for Udp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.socket.local_addr().unwrap())
    }
}
