use openssl::rsa::Rsa;
use std::{
    borrow::BorrowMut,
    cell::RefCell,
    collections::{hash_map::Entry, HashMap},
    fmt::Debug,
    io,
    net::{IpAddr, SocketAddr, UdpSocket},
    sync::{Arc, Mutex},
    thread::spawn,
    time::SystemTime,
};

use crate::{
    encoder::Encoder, Error, Event, InternalPeer, InternalTypeEvent, Message, MessageMethod, Peer,
    RemotePeer,
};

static END: &[u8] = "PL3AZE 5T0P".as_bytes();

macro_rules! info {
    ($($arg:tt)*) => {{
        let res = format!($($arg)*);
        println!("\x1b[32m[INFO]\x1b[0m {}", res);
    }}
}

macro_rules! error {
    ($($arg:tt)*) => {{
        let res = format!($($arg)*);
        println!("\x1b[33m[ERROR]\x1b[0m {}", res);
        panic!("{}", res);
    }}
}

impl Peer {
    /// Return true if the thread of the peer is alive.
    pub fn is_alive(&self) -> bool {
        if let Some(job) = &self.job {
            !job.is_finished()
        } else {
            false
        }
    }

    fn send_message_to(
        socket: &UdpSocket,
        peers: &HashMap<String, RemotePeer>,
        message: Message,
        to: String,
    ) -> Result<String, Error> {
        if let Some(remote) = peers.get(&to) {
            // Encrypt message with remote public key
            let mut remote_message = message.clone();
            Encoder::encrypt(&remote.public_key.clone().unwrap(), &remote_message.data)
                .and_then(|encrypted| {
                    remote_message.data = encrypted;
                    Ok(())
                })
                .unwrap();
            // Split message and send
            for m in remote_message.split() {
                socket
                    .send_to(&m.write(), &remote.addr)
                    .or_else(|e| Err(Error::io(e)))
                    .unwrap();
            }
            Ok(remote.uid.clone())
        } else {
            Err(Error::custom("This UID is not found".to_owned()))
        }
    }

    fn send_message_at(
        socket: &UdpSocket,
        message: &Message,
        to: &SocketAddr,
    ) -> Result<(), Error> {
        // Split and send a basic message
        for m in message.split() {
            socket
                .send_to(&m.write(), to)
                .or_else(|e| Err(Error::io(e)))
                .unwrap();
        }
        Ok(())
    }

    /// Send a message to a specific peer.
    pub fn send_to(&self, message: Message, to: String) -> Result<String, Error> {
        let mut guard_peers = self.peers.lock().unwrap();
        let peers = guard_peers.borrow_mut();
        Self::send_message_to(&self.udp_socket, &peers, message, to)
    }

    /// Send a message to all connected peers, return the remote peer's uids.
    pub fn send_to_all(&self, message: Message) -> Vec<String> {
        let mut remotes = Vec::new();
        let guard_peers = self.peers.lock().unwrap();
        for (uid, _) in guard_peers.clone() {
            Self::send_message_to(&self.udp_socket, &guard_peers.clone(), message.clone(), uid)
                .and_then(|uid| {
                    remotes.push(uid);
                    Ok(())
                })
                .unwrap_or_else(|e| error!("{e}"));
        }
        remotes
    }

    /// Block the peer, remove it from connected peers.
    pub fn block(&self, uid: String) {
        let mut guard_peers = self.peers.lock().unwrap();
        let mut guard_rejects = self.rejects.lock().unwrap();
        guard_rejects.push(uid.clone());
        let peers = guard_peers.borrow_mut();
        if peers.contains_key(&uid) {
            peers.remove(&uid);
        }
    }

    /// Connect to the SocketAddr.
    pub fn connect_to(&self, addr: SocketAddr) {
        let message = Message::internal(
            InternalTypeEvent::CONNECTING,
            MessageMethod::Request,
            &InternalPeer {
                uid: self.uid.clone(),
                public_key_pem: self.public_key_pem.clone(),
                private_key: None,
                addr: self.addr(),
                peers_connected: None,
                rejects: vec![],
                share_peers: self.share_peers,
            },
            None,
        );
        Self::send_message_at(&self.udp_socket, &message, &addr).unwrap();
    }

    pub(crate) fn disconnect_at(&self, addr: &SocketAddr) {
        Self::send_message_at(
            &self.udp_socket,
            &Message::new_internal(
                InternalTypeEvent::DISCONNECTING,
                MessageMethod::Request,
                vec![],
            ),
            addr,
        )
        .unwrap();
    }

    /// Disconnect to the peer.
    pub fn disconnect_to(&self, to: String) {
        let mut guard_peers = self.peers.lock().unwrap();
        let peers = guard_peers.borrow_mut();
        if let Some(remote) = peers.get(&to) {
            self.disconnect_at(&remote.addr);
        } else {
            error!("{} not connected", to);
        }
    }

    /// Disconnect to all remote peers, return the remote peer's uids.
    pub fn disconnect_to_all(&self) -> Vec<String> {
        let mut remotes = Vec::new();
        let guard_peers = self.peers.lock().unwrap();
        for peer in guard_peers.clone() {
            self.disconnect_at(&peer.1.addr);
            remotes.push(peer.0.clone());
        }
        remotes
    }

    /// Get the local address of the peer.
    pub fn addr(&self) -> SocketAddr {
        self.udp_socket.local_addr().unwrap()
    }

    /// Close the peer.
    pub fn close(&self) {
        self.udp_socket
            .send_to(&END.to_vec(), self.addr())
            .or_else(|e| Err(Error::io(e)))
            .unwrap();
    }

    /// Start a new peer on the specific port.
    pub fn start<F>(
        port: u16,
        uid: Option<&str>,
        share_peers: bool,
        observer: F,
    ) -> Result<Peer, Error>
    where
        F: FnMut(Event) -> Option<Message> + Send + Sync + 'static,
    {
        // Build the peer UID
        let uid = Self::build_uid(uid);
        // List of remote peers
        let remote_peers = Arc::new(Mutex::new(HashMap::new()));
        // List of rejects peers
        let rejects = Arc::new(Mutex::new(Vec::new()));
        // Generate SSL keys
        let rsa = Rsa::generate(2048).unwrap();
        // Public Key PEM
        let public_key_pem = rsa.public_key_to_pem().unwrap();
        // Bind socket
        let addr = "127.0.0.1".parse::<IpAddr>().unwrap();
        let socket = UdpSocket::bind(SocketAddr::new(addr, port)).unwrap();
        // Clones for the thread
        let uid_job = uid.clone();
        let socket_job = socket.try_clone().unwrap();
        let public_key_job = public_key_pem.clone();
        let remote_peers_job = Arc::clone(&remote_peers);
        let rejects_peers_job = Arc::clone(&rejects);
        // Start a thread (receive)
        let job = spawn(move || {
            let mut buf = [0; 2048];
            let shared_observer = Arc::new(Mutex::new(RefCell::new(observer)));
            let shared_cache_messages: Arc<Mutex<HashMap<String, Vec<Message>>>> =
                Arc::new(Mutex::new(HashMap::new()));
            // Clones for the loop
            let socket_loop = socket_job.try_clone().unwrap();
            // Loop for receive packets
            loop {
                let mut guard_observer = shared_observer.lock().unwrap();
                let mut guard_cache_messages = shared_cache_messages.lock().unwrap();
                match socket_loop.recv_from(&mut buf) {
                    Ok((number_of_bytes, addr)) => {
                        let data = buf[..number_of_bytes].to_vec();
                        // If stop data
                        if data == END {
                            info!("Peer {} stopped.", uid_job);
                            break;
                        }
                        // Parse Message
                        let message = Message::read(&data).unwrap_or_else(|e| error!("{e}"));
                        let uid_message = message.uid.clone();
                        // Cache for rebuild split messages
                        match guard_cache_messages.entry(uid_message.clone()) {
                            Entry::Occupied(mut o) => {
                                let list = o.get_mut();
                                list.push(message);
                                list
                            }
                            Entry::Vacant(v) => v.insert(vec![message]),
                        };
                        let cache_messages = guard_cache_messages.borrow_mut();
                        let mut guard_remote_peers = remote_peers_job.lock().unwrap();
                        let mut guard_rejects = rejects_peers_job.lock().unwrap();
                        // Parse Event
                        let (event, peers_updated, messages) = Event::parse(
                            InternalPeer {
                                uid: uid_job.clone(),
                                public_key_pem: public_key_job.clone(),
                                private_key: Some(rsa.clone()),
                                addr: socket_job.local_addr().unwrap(),
                                peers_connected: Some(guard_remote_peers.borrow_mut().clone()),
                                rejects: guard_rejects.borrow_mut().clone(),
                                share_peers,
                            },
                            &uid_message,
                            &addr,
                            cache_messages,
                        )
                        .unwrap_or_else(|e| error!("{e}"));
                        // Update connected peers
                        guard_remote_peers.clear();
                        for (uid, remote) in &peers_updated {
                            guard_remote_peers.insert(uid.clone(), remote.clone());
                        }
                        // Internal responses
                        for (remote, internal_messages) in messages {
                            for message in internal_messages {
                                Self::send_message_at(&socket_loop, &message, &remote)
                                    .unwrap_or_else(|e| error!("{e}"));
                            }
                        }
                        // Listener
                        if let Some(event) = event {
                            let from = event.from.clone();
                            let obs = guard_observer.borrow_mut().get_mut();
                            if let Some(response) = obs(event) {
                                Self::send_message_to(&socket_loop, &peers_updated, response, from)
                                    .unwrap();
                            }
                        }
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                        // wait until network socket is ready
                    }
                    Err(e) => error!("{e}"),
                }
            }
        });
        info!("Peer {} started.", uid);
        Ok(Self {
            uid,
            job: Some(job),
            udp_socket: socket,
            peers: remote_peers,
            rejects: rejects,
            public_key_pem,
            share_peers,
        })
    }

    fn build_uid(uid: Option<&str>) -> String {
        let mut default_uid = "P".to_owned();
        default_uid.push_str(
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
                .to_string()
                .as_str(),
        );
        uid.map(|s| s.to_owned()).unwrap_or(default_uid)
    }
}

impl Debug for InternalPeer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InternalPeer")
            .field("uid", &self.uid)
            .field("addr", &self.addr)
            .field("peers_connected", &self.peers_connected)
            .field("share_peers", &self.share_peers)
            .finish()
    }
}

impl Debug for Peer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Peer")
            .field("uid", &self.uid)
            .field("job", &self.job)
            .field("udp_socket", &self.udp_socket)
            .field("peers", &self.peers)
            .finish()
    }
}

impl Debug for RemotePeer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RemotePeer")
            .field("uid", &self.uid)
            .field("addr", &self.addr)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::hash_map::Entry;
    use std::collections::HashMap;
    use std::fmt::Debug;
    use std::thread::sleep;
    use std::{
        sync::{Arc, Mutex},
        time::{Duration, SystemTime},
    };

    use crate::{Message, Peer, TypeEvent};

    pub(crate) fn wait_until<T>(expected: &dyn Fn() -> T, actual: &dyn Fn() -> T)
    where
        T: PartialEq,
        T: Debug,
    {
        let start = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        loop {
            if expected() != actual() {
                sleep(Duration::from_millis(100));
                let current = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_millis();
                if current - start > 3000 {
                    assert_eq!(expected(), actual());
                }
            } else {
                assert_eq!(expected(), actual());
                break;
            }
        }
    }

    fn init_peer(
        uid: &str,
        port: u16,
        peer_connected: Arc<Mutex<Vec<String>>>,
        peer_disconnected: Arc<Mutex<Vec<String>>>,
        messages: Arc<Mutex<HashMap<String, Vec<String>>>>,
        share: bool,
    ) -> Peer {
        Peer::start(port, Some(uid), share, move |event| {
            println!("{:?}", event);
            let mut guard_connected = peer_connected.lock().unwrap();
            let mut guard_disconnected = peer_disconnected.lock().unwrap();
            let mut guard_messages = messages.lock().unwrap();
            if event.type_event == TypeEvent::CONNECTED {
                guard_connected.push(event.from.clone());
                guard_connected.sort();
            }
            if event.type_event == TypeEvent::DISCONNECTED {
                guard_disconnected.push(event.from.clone());
                guard_disconnected.sort();
            }
            if event.type_event == TypeEvent::MESSAGE {
                let text = String::from_utf8(event.message.unwrap().data).unwrap();
                match guard_messages.entry(event.from.clone()) {
                    Entry::Occupied(mut o) => {
                        let list = o.get_mut();
                        list.push(text);
                        list
                    }
                    Entry::Vacant(v) => v.insert(vec![text]),
                };
            }
            None
        })
        .unwrap()
    }

    #[test]
    fn test_connect_share_peers() {
        let peer1_connected = Arc::new(Mutex::new(Vec::new()));
        let peer1 = init_peer(
            "Peer1",
            9001,
            Arc::clone(&peer1_connected),
            Arc::new(Mutex::new(Vec::new())),
            Arc::new(Mutex::new(HashMap::new())),
            true,
        );

        let peer2_connected = Arc::new(Mutex::new(Vec::new()));
        let peer2 = init_peer(
            "Peer2",
            9002,
            Arc::clone(&peer2_connected),
            Arc::new(Mutex::new(Vec::new())),
            Arc::new(Mutex::new(HashMap::new())),
            true,
        );

        let peer3_connected = Arc::new(Mutex::new(Vec::new()));
        let peer3 = init_peer(
            "Peer3",
            9003,
            Arc::clone(&peer3_connected),
            Arc::new(Mutex::new(Vec::new())),
            Arc::new(Mutex::new(HashMap::new())),
            true,
        );

        peer1.connect_to(peer2.addr());
        wait_until(&|| peer1_connected.lock().unwrap().join(","), &|| {
            "Peer2".to_owned()
        });
        wait_until(&|| peer2_connected.lock().unwrap().join(","), &|| {
            "Peer1".to_owned()
        });

        peer3.connect_to(peer2.addr());
        wait_until(&|| peer1_connected.lock().unwrap().join(","), &|| {
            "Peer2,Peer3".to_owned()
        });
        wait_until(&|| peer2_connected.lock().unwrap().join(","), &|| {
            "Peer1,Peer3".to_owned()
        });
        wait_until(&|| peer3_connected.lock().unwrap().join(","), &|| {
            "Peer1,Peer2".to_owned()
        });

        peer1.close();
        peer2.close();
        peer3.close();
    }

    #[test]
    fn test_connect_without_share_peers() {
        let peer1_connected = Arc::new(Mutex::new(Vec::new()));
        let peer1 = init_peer(
            "Peer1",
            9101,
            Arc::clone(&peer1_connected),
            Arc::new(Mutex::new(Vec::new())),
            Arc::new(Mutex::new(HashMap::new())),
            false,
        );

        let peer2_connected = Arc::new(Mutex::new(Vec::new()));
        let peer2 = init_peer(
            "Peer2",
            9102,
            Arc::clone(&peer2_connected),
            Arc::new(Mutex::new(Vec::new())),
            Arc::new(Mutex::new(HashMap::new())),
            false,
        );

        let peer3_connected = Arc::new(Mutex::new(Vec::new()));
        let peer3 = init_peer(
            "Peer3",
            9103,
            Arc::clone(&peer3_connected),
            Arc::new(Mutex::new(Vec::new())),
            Arc::new(Mutex::new(HashMap::new())),
            false,
        );

        peer1.connect_to(peer2.addr());
        peer3.connect_to(peer1.addr());

        wait_until(&|| peer1_connected.lock().unwrap().join(","), &|| {
            "Peer2,Peer3".to_owned()
        });
        wait_until(&|| peer2_connected.lock().unwrap().join(","), &|| {
            "Peer1".to_owned()
        });
        wait_until(&|| peer3_connected.lock().unwrap().join(","), &|| {
            "Peer1".to_owned()
        });

        peer1.close();
        peer2.close();
        peer3.close();
    }

    #[test]
    fn test_disconnect() {
        let peer1_disconnected = Arc::new(Mutex::new(Vec::new()));
        let peer1 = init_peer(
            "Peer1",
            9201,
            Arc::new(Mutex::new(Vec::new())),
            Arc::clone(&peer1_disconnected),
            Arc::new(Mutex::new(HashMap::new())),
            false,
        );

        let peer2_connected = Arc::new(Mutex::new(Vec::new()));
        let peer2_disconnected = Arc::new(Mutex::new(Vec::new()));
        let peer2 = init_peer(
            "Peer2",
            9202,
            Arc::clone(&peer2_connected),
            Arc::clone(&peer2_disconnected),
            Arc::new(Mutex::new(HashMap::new())),
            false,
        );

        peer1.connect_to(peer2.addr());

        wait_until(&|| peer2_connected.lock().unwrap().join(","), &|| {
            "Peer1".to_owned()
        });

        peer2.disconnect_to("Peer1".to_owned());

        wait_until(&|| peer1_disconnected.lock().unwrap().join(","), &|| {
            "Peer2".to_owned()
        });
        wait_until(&|| peer2_disconnected.lock().unwrap().join(","), &|| {
            "Peer1".to_owned()
        });

        peer1.close();
        peer2.close();
    }

    #[test]
    fn test_close() {
        let peer = init_peer(
            "Peer1",
            9300,
            Arc::new(Mutex::new(Vec::new())),
            Arc::new(Mutex::new(Vec::new())),
            Arc::new(Mutex::new(HashMap::new())),
            false,
        );
        peer.close();
        wait_until(&|| false, &|| peer.is_alive());
    }

    #[test]
    fn test_send_message() {
        let peer1_connected = Arc::new(Mutex::new(Vec::new()));
        let peer1 = init_peer(
            "Peer1",
            9401,
            Arc::clone(&peer1_connected),
            Arc::new(Mutex::new(Vec::new())),
            Arc::new(Mutex::new(HashMap::new())),
            false,
        );
        let peer2_messages = Arc::new(Mutex::new(HashMap::new()));
        let peer2 = init_peer(
            "Peer2",
            9402,
            Arc::new(Mutex::new(Vec::new())),
            Arc::new(Mutex::new(Vec::new())),
            Arc::clone(&peer2_messages),
            false,
        );

        peer1.connect_to(peer2.addr());
        wait_until(&|| peer1_connected.lock().unwrap().join(","), &|| {
            "Peer2".to_owned()
        });

        peer1
            .send_to(Message::new("Hello !".as_bytes().to_vec()), peer2.uid)
            .unwrap();
        wait_until(&|| true, &|| {
            peer2_messages.lock().unwrap().get(&peer1.uid).is_some()
        });
        wait_until(
            &|| {
                peer2_messages
                    .lock()
                    .unwrap()
                    .get(&peer1.uid)
                    .unwrap()
                    .join(",")
            },
            &|| "Hello !".to_owned(),
        );
    }

    #[test]
    fn test_send_message_to_all() {
        let peer1_connected = Arc::new(Mutex::new(Vec::new()));
        let peer1 = init_peer(
            "Peer1",
            9501,
            Arc::clone(&peer1_connected),
            Arc::new(Mutex::new(Vec::new())),
            Arc::new(Mutex::new(HashMap::new())),
            true,
        );
        let peer2_messages = Arc::new(Mutex::new(HashMap::new()));
        let peer2 = init_peer(
            "Peer2",
            9502,
            Arc::new(Mutex::new(Vec::new())),
            Arc::new(Mutex::new(Vec::new())),
            Arc::clone(&peer2_messages),
            true,
        );
        let peer3_messages = Arc::new(Mutex::new(HashMap::new()));
        let peer3 = init_peer(
            "Peer3",
            9503,
            Arc::new(Mutex::new(Vec::new())),
            Arc::new(Mutex::new(Vec::new())),
            Arc::clone(&peer3_messages),
            true,
        );

        peer1.connect_to(peer2.addr());
        peer3.connect_to(peer2.addr());
        wait_until(&|| peer1_connected.lock().unwrap().join(","), &|| {
            "Peer2,Peer3".to_owned()
        });

        let mut remotes = peer1.send_to_all(Message::new("Hello !".as_bytes().to_vec()));
        remotes.sort();
        assert_eq!(vec!["Peer2", "Peer3"], remotes);

        wait_until(&|| true, &|| {
            peer2_messages.lock().unwrap().get(&peer1.uid).is_some()
        });
        wait_until(&|| true, &|| {
            peer3_messages.lock().unwrap().get(&peer1.uid).is_some()
        });
        wait_until(
            &|| {
                peer2_messages
                    .lock()
                    .unwrap()
                    .get(&peer1.uid)
                    .unwrap()
                    .join(",")
            },
            &|| "Hello !".to_owned(),
        );
        wait_until(
            &|| {
                peer3_messages
                    .lock()
                    .unwrap()
                    .get(&peer1.uid)
                    .unwrap()
                    .join(",")
            },
            &|| "Hello !".to_owned(),
        );
    }

    #[test]
    fn test_event_chaining() {
        let peer1_message = Arc::new(Mutex::new(Vec::new()));
        let shared_peer1_message = Arc::clone(&peer1_message);
        let peer1 = Peer::start(9601, Some("Peer1"), true, move |event| {
            if event.type_event == TypeEvent::CONNECTED {
                Some(Message::new("Hello I am Peer1".as_bytes().to_vec()))
            } else if event.type_event == TypeEvent::MESSAGE {
                let mut guard_message = shared_peer1_message.lock().unwrap();
                guard_message.push(String::from_utf8(event.message.unwrap().data).unwrap());
                None
            } else {
                None
            }
        })
        .unwrap();
        let peer2_message = Arc::new(Mutex::new(Vec::new()));
        let shared_peer2_message = Arc::clone(&peer2_message);
        let peer2 = Peer::start(9602, Some("Peer2"), true, move |event| {
            if event.type_event == TypeEvent::MESSAGE {
                let mut guard_message = shared_peer2_message.lock().unwrap();
                guard_message.push(String::from_utf8(event.message.unwrap().data).unwrap());
                Some(Message::new("Hello I am Peer2".as_bytes().to_vec()))
            } else {
                None
            }
        })
        .unwrap();
        peer1.connect_to(peer2.addr());
        wait_until(&|| false, &|| peer2_message.lock().unwrap().is_empty());
        wait_until(&|| "Hello I am Peer1".to_owned(), &|| {
            peer2_message.lock().unwrap().get(0).unwrap().clone()
        });
        wait_until(&|| false, &|| peer1_message.lock().unwrap().is_empty());
        wait_until(&|| "Hello I am Peer2".to_owned(), &|| {
            peer1_message.lock().unwrap().get(0).unwrap().clone()
        });
    }
}
