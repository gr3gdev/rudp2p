use crate::{
    configuration::Configuration, dao::PeerDao, network::*, observer::Observer, thread,
    utils::unwrap::unwrap_result,
};
use std::{
    fmt::Debug,
    net::{IpAddr, SocketAddr, UdpSocket},
    sync::{Arc, Mutex},
};

#[derive(Debug)]
#[doc = include_str!("../README.md")]
pub struct Peer {
    /// UDP socket.
    udp_socket: UdpSocket,
    /// The configuration.
    configuration: Configuration,
    /// Custom DAO.
    dao: Arc<Mutex<dyn PeerDao>>,
}

impl Clone for Peer {
    fn clone(&self) -> Self {
        Self {
            udp_socket: unwrap_result(self.udp_socket.try_clone(), "Unable to clone socket"),
            configuration: self.configuration.clone(),
            dao: Arc::clone(&self.dao),
        }
    }
}

impl Peer {
    /// Create a new Peer.
    pub async fn new<O, D>(configuration: Configuration, dao: D, observer: O) -> Peer
    where
        O: Observer,
        D: PeerDao,
    {
        // Get local IP
        let addr = unwrap_result(
            "127.0.0.1".parse::<IpAddr>(),
            "Unable to initialize IP address",
        );

        // New UDP socket
        let socket = unwrap_result(
            UdpSocket::bind(SocketAddr::new(addr, configuration.port)),
            "Unable to bind socket on port",
        );

        let dao = Arc::new(Mutex::new(dao));

        // Start thread for processing messages
        let instance = thread::start_socket_job(&configuration, &socket, &dao, observer).await;

        // Return Peer
        let peer = Peer {
            udp_socket: socket,
            configuration: instance.configuration,
            dao,
        };
        log::trace!("Peer::new({:?}, observer) => {:?}", configuration, peer);
        peer
    }

    /// Return true if the Peer is alive.
    pub async fn is_alive(&self) -> bool {
        let alive = self.dao.lock().unwrap().find_status().await;
        log::trace!("Peer::is_alive() => {alive}");
        alive
    }

    /// Connect to another Peer with his address.
    #[cfg(not(feature = "ssl"))]
    pub fn connect_to(&self, addr: &SocketAddr) -> () {
        log::trace!("Peer::connect_to({addr})");
        let request = Request::new_connection(&self.configuration);
        request.send(&self.udp_socket, &addr);
    }

    /// Connect to another Peer with his address.
    #[cfg(feature = "ssl")]
    pub fn connect_to(&self, addr: &SocketAddr) -> () {
        log::trace!("Peer::connect_to({addr})");
        let request = Request::new_connection(&self.configuration);
        request.send(&self.udp_socket, &addr, &vec![]);
    }

    /// Send a request to the peers.
    #[cfg(not(feature = "ssl"))]
    fn send(&self, request: Request, remote_peers: Vec<RemotePeer>) -> () {
        log::trace!("Peer::send({:?}, {:?})", request, remote_peers);
        for remote in remote_peers {
            request.send(&self.udp_socket, &remote.addr);
        }
    }

    /// Send a request to the peers.
    #[cfg(feature = "ssl")]
    fn send(&self, request: Request, remote_peers: Vec<RemotePeer>) -> () {
        log::trace!("Peer::send({:?}, {:?})", request, remote_peers);
        for remote in remote_peers {
            request.send(&self.udp_socket, &remote.addr, &remote.public_key);
        }
    }

    /// Send a request to another Peer with his address.
    pub async fn send_to(&self, request: Request, remote_address: &SocketAddr) -> () {
        log::trace!("Peer::send_to({:?}, {remote_address})", request);
        let request = Request::new_message(&request.content);
        let remotes = self
            .dao
            .lock()
            .unwrap()
            .find_remotes_by_address(&remote_address)
            .await;
        self.send(request, remotes);
    }

    /// Send a request to all Peers connected.
    pub async fn send_to_all(&self, request: &Request) -> () {
        log::trace!("Peer::send_to_all({:?})", request);
        let request = Request::new_message(&request.content);
        let remote_peers = self.dao.lock().unwrap().find_all_remotes().await;
        self.send(request, remote_peers);
    }

    /// Disconnect to another Peer with his address.
    pub async fn disconnect_to(&self, remote_address: &SocketAddr) -> () {
        log::trace!("Peer::disconnect_to({remote_address})");
        let request = Request::new_disconnection();
        let remotes = self
            .dao
            .lock()
            .unwrap()
            .find_remotes_by_address(&remote_address)
            .await;
        self.send(request, remotes);
    }

    /// Disconnect to all Peers connected.
    pub async fn disconnect_to_all(&self) -> () {
        log::trace!("Peer::disconnect_to_all()");
        let request = Request::new_disconnection();
        let remote_peers = self.dao.lock().unwrap().find_all_remotes().await;
        self.send(request, remote_peers);
    }

    /// Get local address of the sockets.
    pub fn addr(&self) -> SocketAddr {
        let addr = unwrap_result(
            self.udp_socket.local_addr(),
            "Unable to get the local address",
        );
        log::trace!("Peer::addr() => {addr}");
        addr
    }

    /// Block a remote peer.
    pub async fn block(&self, remote_address: &SocketAddr) -> () {
        log::trace!("Peer::block({remote_address})");
        let remotes = self
            .dao
            .lock()
            .unwrap()
            .find_remotes_by_address(&remote_address)
            .await;
        for remote in remotes {
            self.disconnect_to(remote_address).await;
            if self.dao.lock().unwrap().block(&remote.addr).await < 1 {
                log::error!("[DAO] Unable to block {:?}", remote);
            }
        }
    }

    /// Unblock a remote peer.
    pub async fn unblock(&self, remote_address: &SocketAddr) -> () {
        log::trace!("Peer::unblock({remote_address})");
        let blocked_addresses = self.dao.lock().unwrap().find_all_block().await;
        if blocked_addresses.contains(remote_address) {
            if self.dao.lock().unwrap().unblock(&remote_address).await < 1 {
                log::error!("[DAO] Unable to unblock {}", remote_address);
            } else {
                self.connect_to(remote_address);
            }
        }
    }

    /// Close the Peer and stop the socket.
    pub fn close(&self) -> () {
        log::trace!("Peer::close()");
        thread::stop_job(&self.udp_socket);
    }
}

/// # RemotePeer
///
/// A another Peer which is connected with the local Peer.
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct RemotePeer {
    /// The remote address.
    pub addr: SocketAddr,
    #[cfg(feature = "ssl")]
    pub(crate) public_key: Vec<u8>,
}

impl Debug for RemotePeer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RemotePeer")
            .field("addr", &self.addr)
            .finish()
    }
}

impl RemotePeer {
    #[cfg(not(feature = "ssl"))]
    pub fn new(addr: &SocketAddr) -> Self {
        Self { addr: addr.clone() }
    }

    #[cfg(feature = "ssl")]
    pub fn new(addr: &SocketAddr, public_key: &Vec<u8>) -> Self {
        Self {
            addr: addr.clone(),
            public_key: public_key.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Peer, RemotePeer};
    use crate::{
        configuration::Configuration,
        network::{events::Message, Request, Response},
        observer::Observer,
    };
    use async_trait::async_trait;
    use futures::executor::block_on;
    use serialize_bits::des::DeserializerData;
    use std::{
        collections::HashMap,
        fmt::Debug,
        net::SocketAddr,
        sync::{Arc, Mutex},
        time::{Duration, SystemTime},
    };

    #[cfg(not(feature = "ssl"))]
    fn prepare(port: u16) -> (Peer, Test) {
        use crate::dao::InMemoryDao;

        let conf = Configuration::builder()
            .port(port)
            .share_connections(true)
            .build();
        let test = Test::default();
        let peer = block_on(Peer::new(conf, InMemoryDao::default(), test.clone()));
        (peer, test)
    }

    #[cfg(feature = "ssl")]
    fn prepare(port: u16) -> (Peer, Test) {
        use crate::{configuration::SSL, dao::InMemoryDao};

        let conf = Configuration::builder(SSL::from_size(4096))
            .port(port)
            .share_connections(true)
            .build();
        let test = Test::default();
        let peer = block_on(Peer::new(conf, InMemoryDao::default(), test.clone()));
        (peer, test)
    }

    #[derive(Clone, Default)]
    struct Test {
        connections: Arc<Mutex<Vec<RemotePeer>>>,
        disconnections: Arc<Mutex<Vec<RemotePeer>>>,
        messages: Arc<Mutex<HashMap<SocketAddr, Vec<String>>>>,
    }

    #[async_trait]
    impl Observer for Test {
        async fn on_connected(&mut self, remote: &RemotePeer) -> Option<Response> {
            let mut connections = self.connections.lock().unwrap();
            connections.push(remote.clone());
            None
        }
        async fn on_disconnected(&mut self, remote: &RemotePeer) -> Option<Response> {
            let mut disconnections = self.disconnections.lock().unwrap();
            disconnections.push(remote.clone());
            None
        }
        async fn on_message(&mut self, m: &Message) -> Option<Response> {
            let mut messages = self.messages.lock().unwrap();
            match messages.entry(m.from.addr) {
                std::collections::hash_map::Entry::Occupied(mut o) => {
                    let list: &mut Vec<String> = o.get_mut();
                    let (value, _) = String::from_data(&m.content, 0);
                    list.push(value);
                }
                std::collections::hash_map::Entry::Vacant(v) => {
                    let (value, _) = String::from_data(&m.content, 0);
                    v.insert(vec![value]);
                }
            }
            None
        }
    }

    fn wait_while_condition(condition: &dyn Fn() -> bool) {
        let start = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        while condition() {
            std::thread::sleep(Duration::from_millis(100));
            let current = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_millis();
            if current - start > 10000 {
                panic!("Timeout !");
            }
        }
    }

    fn check<T>(expected: Vec<T>, mut actual: Vec<T>)
    where
        T: Ord,
        T: Debug,
    {
        actual.sort();
        assert_eq!(expected, actual);
    }

    #[test]
    fn validate() {
        //env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("trace")).init();
        let (peer1, test1) = prepare(9901);
        let (peer2, test2) = prepare(9902);
        let (peer3, test3) = prepare(9903);

        // Share connections
        // P1 => P2
        peer1.connect_to(&peer2.addr());
        // P3 => P2 ... P2 =(P1)=> P3 ... P3 => P1
        peer3.connect_to(&peer2.addr());
        wait_while_condition(&|| {
            test1.connections.lock().unwrap().len() < 2
                || test2.connections.lock().unwrap().len() < 2
                || test3.connections.lock().unwrap().len() < 2
        });
        check(
            vec![peer2.addr(), peer3.addr()],
            test1
                .connections
                .lock()
                .unwrap()
                .iter()
                .map(|r| r.addr)
                .collect(),
        );
        check(
            vec![peer1.addr(), peer3.addr()],
            test2
                .connections
                .lock()
                .unwrap()
                .iter()
                .map(|r| r.addr)
                .collect(),
        );
        check(
            vec![peer1.addr(), peer2.addr()],
            test3
                .connections
                .lock()
                .unwrap()
                .iter()
                .map(|r| r.addr)
                .collect(),
        );

        // Send message to all
        block_on(peer2.send_to_all(&Request::new(String::from("Hello everybody !"))));
        wait_while_condition(&|| {
            test1.messages.lock().unwrap().len() < 1 || test3.messages.lock().unwrap().len() < 1
        });
        check(
            vec![(peer2.addr(), vec![String::from("Hello everybody !")])],
            Vec::from_iter(test1.messages.lock().unwrap().clone()),
        );
        check(
            vec![(peer2.addr(), vec![String::from("Hello everybody !")])],
            Vec::from_iter(test3.messages.lock().unwrap().clone()),
        );

        let binding = test2.connections.lock().unwrap();
        let remote_peer1 = binding.iter().find(|r| r.addr == peer1.addr()).unwrap();

        // Send message to peer
        block_on(peer2.send_to(Request::new(String::from("What's your name ?")), &remote_peer1.addr));
        wait_while_condition(&|| {
            test1
                .messages
                .lock()
                .unwrap()
                .get(&peer2.addr())
                .unwrap()
                .len()
                < 2
        });
        check(
            vec![(
                peer2.addr(),
                vec![
                    String::from("Hello everybody !"),
                    String::from("What's your name ?"),
                ],
            )],
            Vec::from_iter(test1.messages.lock().unwrap().clone()),
        );

        // Disconnection with all
        block_on(peer2.disconnect_to_all());
        wait_while_condition(&|| {
            test1.disconnections.lock().unwrap().len() < 1
                || test3.disconnections.lock().unwrap().len() < 1
        });
        check(
            vec![peer2.addr()],
            test1
                .disconnections
                .lock()
                .unwrap()
                .iter()
                .map(|r| r.addr)
                .collect(),
        );
        check(
            vec![peer2.addr()],
            test3
                .disconnections
                .lock()
                .unwrap()
                .iter()
                .map(|r| r.addr)
                .collect(),
        );

        let binding = test1.connections.lock().unwrap();
        let remote_peer3 = binding.iter().find(|r| r.addr == peer3.addr()).unwrap();

        // Disconnection with peer
        block_on(peer1.disconnect_to(&remote_peer3.addr));
        wait_while_condition(&|| test3.disconnections.lock().unwrap().len() < 2);
        check(
            vec![peer1.addr(), peer2.addr()],
            test3
                .disconnections
                .lock()
                .unwrap()
                .iter()
                .map(|r| r.addr)
                .collect(),
        );
    }
}
