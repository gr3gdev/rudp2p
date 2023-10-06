use crate::{configuration::Configuration, dao, network::*, observer::Observer, thread};
use std::{
    fmt::Debug,
    net::{IpAddr, SocketAddr, UdpSocket},
    sync::Arc,
};

/// # Peer
///
#[cfg_attr(
    all(feature = "sqlite", not(feature = "ssl")),
    doc = r##"
Example of a connection with the sqlite feature
```
use async_trait::async_trait;
use rudp2plib::{configuration::*, network::{*, events::Message}, observer::*, peer::*};

struct MyObserver {
    name: String,
}

#[async_trait]
impl Observer for MyObserver {
    async fn on_connected(&mut self, remote: &RemotePeer) -> Option<Response> {
        let mut text = String::from("Hello I am ");
        text.push_str(&self.name);
        Some(Response::text(&text))
    }

    async fn on_disconnected(&mut self, remote: &RemotePeer) -> Option<Response> {
        Some(Response::text("Goodbye !"))
    }

    async fn on_message(&mut self, m: &Message) -> Option<Response> {
        println!("{} : {}", self.name, String::from_utf8(m.content.clone()).unwrap());
        None
    }
}

async fn example() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let observer1 = MyObserver{
        name: String::from("Peer1"),
    };
    let peer1 = Peer::new(
        Configuration::builder().port(9001).build(),
        observer1,
    ).await;

    let observer2 = MyObserver{
        name: String::from("Peer2"),
    };
    let peer2 = Peer::new(
        Configuration::builder().port(9002).build(),
        observer2,
    ).await;

    peer1.connect_to(&peer2.addr());

    peer1.close();
    peer2.close();
}

fn main() {
    futures::executor::block_on(example());
}
```
"##
)]
pub struct Peer {
    /// UDP socket.
    udp_socket: UdpSocket,
    /// The configuration.
    configuration: Configuration,
    /// Database pool.
    pool: Arc<dao::Pool>,
}

impl Debug for Peer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Peer")
            .field("udp_socket", &self.udp_socket)
            .field("configuration", &self.configuration)
            .field("pool", &self.pool)
            .finish()
    }
}

impl Clone for Peer {
    fn clone(&self) -> Self {
        Self {
            udp_socket: self.udp_socket.try_clone().expect("Unable to clone socket"),
            configuration: self.configuration.clone(),
            pool: self.pool.clone(),
        }
    }
}

impl Peer {
    /// Create a new Peer.
    pub async fn new<O>(configuration: Configuration, observer: O) -> Peer
    where
        O: Observer,
    {
        // Get local IP
        let addr = "127.0.0.1"
            .parse::<IpAddr>()
            .or_else(|e| {
                log::error!("Unable to initialize IP address : {e}");
                Err("Unable to initialize IP address")
            })
            .unwrap();

        // New UDP socket
        let socket = UdpSocket::bind(SocketAddr::new(addr, configuration.port))
            .or_else(|e| {
                log::error!("Unable to bind socket on port : {e}");
                Err("Unable to bind socket on port")
            })
            .unwrap();

        // Start thread for processing messages
        let instance = thread::start_socket_job(&configuration, &socket, observer).await;

        // Return Peer
        let peer = Peer {
            udp_socket: socket,
            configuration: instance.configuration,
            pool: instance.pool,
        };
        log::trace!("Peer::new({:?}, observer) => {:?}", configuration, peer);
        peer
    }

    /// Return true if the Peer is alive.
    pub async fn is_alive(&self) -> bool {
        let alive = dao::thread::status(&self.pool).await;
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
        let remotes = dao::remote::select_by_address(&self.pool, &remote_address).await;
        self.send(request, remotes);
    }

    /// Send a request to all Peers connected.
    pub async fn send_to_all(&self, request: &Request) -> () {
        log::trace!("Peer::send_to_all({:?})", request);
        let request = Request::new_message(&request.content);
        let remote_peers = dao::remote::select_all(&self.pool).await;
        self.send(request, remote_peers);
    }

    /// Disconnect to another Peer with his address.
    pub async fn disconnect_to(&self, remote_address: &SocketAddr) -> () {
        log::trace!("Peer::disconnect_to({remote_address})");
        let request = Request::new_disconnection();
        let remotes = dao::remote::select_by_address(&self.pool, &remote_address).await;
        self.send(request, remotes);
    }

    /// Disconnect to all Peers connected.
    pub async fn disconnect_to_all(&self) -> () {
        log::trace!("Peer::disconnect_to_all()");
        let request = Request::new_disconnection();
        let remote_peers = dao::remote::select_all(&self.pool).await;
        self.send(request, remote_peers);
    }

    /// Get local address of the sockets.
    pub fn addr(&self) -> SocketAddr {
        let addr = self.udp_socket.local_addr().unwrap();
        log::trace!("Peer::addr() => {addr}");
        addr
    }

    /// Block a remote peer.
    pub async fn block(&self, remote_address: &SocketAddr) -> () {
        log::trace!("Peer::block({remote_address})");
        let remotes = dao::remote::select_by_address(&self.pool, &remote_address).await;
        for remote in remotes {
            self.disconnect_to(remote_address).await;
            if dao::block::add(&self.pool, &remote.addr).await < 1 {
                log::error!("[DAO] Unable to block {:?}", remote);
            }
        }
    }

    /// Unblock a remote peer.
    pub async fn unblock(&self, remote_address: &SocketAddr) -> () {
        log::trace!("Peer::unblock({remote_address})");
        let blocked_addresses = dao::block::select_all(&self.pool).await;
        if blocked_addresses.contains(remote_address) {
            if dao::block::remove(&self.pool, &remote_address).await < 1 {
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
    pub(crate) id: i64,
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

#[cfg(test)]
#[cfg(feature = "sqlite")]
mod tests {
    use super::{Peer, RemotePeer};
    use crate::{
        configuration::Configuration,
        network::{events::Message, Request, Response},
        observer::Observer,
    };
    use async_trait::async_trait;
    use futures::executor::block_on;
    use std::{
        collections::HashMap,
        fmt::Debug,
        net::SocketAddr,
        sync::{Arc, Mutex},
        time::{Duration, SystemTime},
    };

    #[cfg(not(feature = "ssl"))]
    fn prepare(port: u16) -> (Peer, Test) {
        let conf = Configuration::builder()
            .port(port)
            .share_connections(true)
            .build();
        let test = Test::default();
        let peer = block_on(Peer::new(conf, test.clone()));
        (peer, test)
    }

    #[cfg(feature = "ssl")]
    fn prepare(port: u16) -> (Peer, Test) {
        use crate::configuration::SSL;

        let conf = Configuration::builder(SSL::from_size(4096))
            .port(port)
            .share_connections(true)
            .build();
        let test = Test::default();
        let peer = block_on(Peer::new(conf, test.clone()));
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
                    list.push(String::from_utf8(m.content.clone()).unwrap());
                }
                std::collections::hash_map::Entry::Vacant(v) => {
                    v.insert(vec![String::from_utf8(m.content.clone()).unwrap()]);
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
        block_on(peer2.send_to_all(&Request::new("Hello everybody !")));
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
        block_on(peer2.send_to(Request::new("What's your name ?"), &remote_peer1.addr));
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
