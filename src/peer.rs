use crate::{
    configuration::Configuration,
    dao::{block, remote, thread, Pool},
    network::*,
    observer::Observer,
    thread::{start_socket_job, stop_job},
};
use log::{debug, error, info};
use std::{
    fmt::Debug,
    net::{IpAddr, SocketAddr, UdpSocket},
};

/// # Peer
///
/// Example of a connection
/// ```
/// use async_trait::async_trait;
/// use rudp2plib::{configuration::*, network::{*, events::Message}, observer::*, peer::*};
///
/// struct MyObserver {
///     name: String,
/// }
///
/// #[async_trait]
/// impl Observer for MyObserver {
///     async fn on_connected(&mut self, remote: &RemotePeer) -> Option<Response> {
///         let mut text = String::from("Hello I am ");
///         text.push_str(&self.name);
///         Some(Response::text(&text))
///     }
///
///     async fn on_disconnected(&mut self, remote: &RemotePeer) -> Option<Response> {
///         Some(Response::text("Goodbye !"))
///     }
///
///     async fn on_message(&mut self, m: &Message) -> Option<Response> {
///         println!("{} : {}", self.name, String::from_utf8(m.content.clone()).unwrap());
///         None
///     }
/// }
///
/// async fn example() {
///     env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
///
///     let observer1 = MyObserver{
///         name: String::from("Peer1"),
///     };
///     let peer1 = Peer::new(
///         Configuration::builder().port(9001).build(),
///         observer1,
///     ).await;
///
///     let observer2 = MyObserver{
///         name: String::from("Peer2"),
///     };
///     let peer2 = Peer::new(
///         Configuration::builder().port(9002).build(),
///         observer2,
///     ).await;
///
///     peer1.connect_to(&peer2.addr());
///
///     peer1.close();
///     peer2.close();
/// }
///
/// fn main() {
///     futures::executor::block_on(example());
/// }
/// ```
///
pub struct Peer {
    /// UDP socket.
    udp_socket: UdpSocket,
    /// PEM of the public key for remote encryption.
    public_key_pem: Vec<u8>,
    /// Database pool.
    pool: Pool,
}

impl Debug for Peer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Peer")
            .field("udp_socket", &self.udp_socket)
            .field("public_key_pem", &self.public_key_pem.len())
            .field("pool", &self.pool)
            .finish()
    }
}

impl Clone for Peer {
    fn clone(&self) -> Self {
        Self {
            udp_socket: self.udp_socket.try_clone().expect("Unable to clone socket"),
            public_key_pem: self.public_key_pem.clone(),
            pool: self.pool.clone(),
        }
    }
}

impl Peer {
    pub async fn new<O>(configuration: Configuration, observer: O) -> Peer
    where
        O: Observer,
    {
        // Get local IP
        let addr = "127.0.0.1"
            .parse::<IpAddr>()
            .or_else(|e| {
                error!("Unable to initialize IP address : {e}");
                Err("Unable to initialize IP address")
            })
            .unwrap();

        // New UDP socket
        let socket = UdpSocket::bind(SocketAddr::new(addr, configuration.port))
            .or_else(|e| {
                error!("Unable to bind socket on port : {e}");
                Err("Unable to bind socket on port")
            })
            .unwrap();

        // Start thread for processing messages
        let instance = start_socket_job(&configuration, &socket, observer).await;

        // Return Peer
        Peer {
            udp_socket: socket,
            public_key_pem: instance.public_key,
            pool: instance.pool,
        }
    }

    pub async fn is_alive(&self) -> bool {
        thread::status(&self.pool).await
    }

    pub fn connect_to(&self, addr: &SocketAddr) -> () {
        info!("connect to {}", addr);
        let request = Request::new_connection(&self.public_key_pem);
        request.send(&self.udp_socket, &addr, &vec![]);
    }

    fn send(&self, request: Request, remote_peers: Vec<RemotePeer>) -> () {
        for remote in remote_peers {
            debug!("Send {:?} to {:?}", request, remote);
            request.send(&self.udp_socket, &remote.addr, &remote.public_key);
        }
    }

    pub async fn send_to(&self, request: Request, remote_address: &SocketAddr) -> () {
        let request = Request::new_message(&request.content);
        let remotes = remote::select_by_address(&self.pool, &remote_address).await;
        self.send(request, remotes);
    }

    pub async fn send_to_all(&self, request: &Request) -> () {
        let request = Request::new_message(&request.content);
        let remote_peers = remote::select_all(&self.pool).await;
        self.send(request, remote_peers);
    }

    pub async fn disconnect_to(&self, remote_address: &SocketAddr) -> () {
        let request = Request::new_disconnection();
        let remotes = remote::select_by_address(&self.pool, &remote_address).await;
        self.send(request, remotes);
    }

    pub async fn disconnect_to_all(&self) -> () {
        let request = Request::new_disconnection();
        let remote_peers = remote::select_all(&self.pool).await;
        self.send(request, remote_peers);
    }

    pub fn addr(&self) -> SocketAddr {
        self.udp_socket.local_addr().unwrap()
    }

    pub async fn block(&self, remote_address: &SocketAddr) -> () {
        let remotes = remote::select_by_address(&self.pool, &remote_address).await;
        for remote in remotes {
            debug!("Block {remote_address}");
            self.disconnect_to(remote_address).await;
            block::add(&self.pool, &remote.addr).await;
        }
    }

    pub async fn unblock(&self, remote_address: &SocketAddr) -> () {
        let blocked_addresses = block::select_all(&self.pool).await;
        if blocked_addresses.contains(remote_address) {
            debug!("Unblock {remote_address}");
            block::remove(&self.pool, &remote_address).await;
            self.connect_to(remote_address);
        }
    }

    pub fn close(&self) -> () {
        stop_job(&self.udp_socket);
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct RemotePeer {
    pub(crate) id: i64,
    pub addr: SocketAddr,
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
    use super::{RemotePeer, Peer};
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

    fn prepare(port: u16) -> (Peer, Test) {
        let conf = Configuration::builder()
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
            std::thread::sleep(Duration::from_millis(1000));
            let current = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_millis();
            if current - start > 5000 {
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
