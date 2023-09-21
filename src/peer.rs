use crate::{
    configuration::Configuration,
    dao::{
        block,
        remote::{self, RemotePeer},
        thread,
    },
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
/// use rudp2plib::{configuration::*, network::{events::*, *}, observer::*, peer::*};
///
/// struct MyObserver {
///     name: String,
///     messages: Vec<String>,
/// }
///
/// #[async_trait]
/// impl Observer for MyObserver {
///     async fn on_connected(&mut self, c: Connected) -> Option<Response> {
///         let mut text = String::from("Hello I am ");
///         text.push_str(&self.name);
///         Some(Response::text(&text))
///     }
///
///     async fn on_disconnected(&mut self, d: Disconnected) -> Option<Response> {
///         Some(Response::text("Goodbye !"))
///     }
///
///     async fn on_message(&mut self, m: Message) -> Option<Response> {
///         self.messages.push(String::from_utf8(m.content).unwrap());
///         None
///     }
/// }
///
/// async fn example() {
///     env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
///
///     let peer1 = Peer::new(
///         Configuration::builder().port(9001).name("Peer1").build(),
///         MyObserver{
///             name: String::from("Peer1"),
///             messages: Vec::new(),
///         },
///     ).await;
///
///     let peer2 = Peer::new(
///         Configuration::builder().port(9002).name("Peer2").build(),
///         MyObserver{
///             name: String::from("Peer2"),
///             messages: Vec::new(),
///         },
///     ).await;
///
///     peer1.connect_to(peer2.addr());
///     std::thread::sleep(std::time::Duration::from_millis(1000));
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
    /// Unique identifier.
    pub uid: String,
    /// UDP socket.
    udp_socket: UdpSocket,
    /// PEM of the public key for remote encryption.
    public_key_pem: Vec<u8>,
    /// Database pool.
    pool: r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>,
}

impl Clone for Peer {
    fn clone(&self) -> Self {
        Self {
            uid: self.uid.clone(),
            udp_socket: self.udp_socket.try_clone().expect("Unable to clone socket"),
            public_key_pem: self.public_key_pem.clone(),
            pool: self.pool.clone(),
        }
    }
}

impl Debug for Peer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Peer").field("uid", &self.uid).finish()
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
            uid: instance.uid,
            udp_socket: socket,
            public_key_pem: instance.public_key,
            pool: instance.pool,
        }
    }

    pub async fn is_alive(&self) -> bool {
        thread::status(&self.uid, &self.pool).await
    }

    pub fn connect_to(&self, addr: SocketAddr) -> () {
        info!("[PEER {}] connect to {}", self.uid, addr);
        let request = Request::new_connection(&self.uid, &self.public_key_pem);
        request.send(&self.udp_socket, &addr, &vec![]);
    }

    fn send(&self, request: Request, remote_peers: Vec<RemotePeer>) -> () {
        for remote in remote_peers {
            debug!("[PEER {}] send {:?} to {:?}", self.uid, request, remote);
            request.send(&self.udp_socket, &remote.addr, &remote.public_key);
        }
    }

    pub async fn send_to(&self, request: Request, peer: String) -> () {
        let request = Request::new_message(&self.uid, &request.content);
        let remote_peers = remote::select_by_uid(&self.uid, &self.pool, &peer).await;
        self.send(request, remote_peers);
    }

    pub async fn send_to_all(&self, request: Request) -> () {
        let request = Request::new_message(&self.uid, &request.content);
        let remote_peers = remote::select_all(&self.uid, &self.pool).await;
        self.send(request, remote_peers);
    }

    pub async fn disconnect_to(&self, peer: String) -> () {
        let request = Request::new_disconnection(&self.uid);
        let remote_peers = remote::select_by_uid(&self.uid, &self.pool, &peer).await;
        self.send(request, remote_peers);
    }

    pub async fn disconnect_to_all(&self) -> () {
        let request = Request::new_disconnection(&self.uid);
        let remote_peers = remote::select_all(&self.uid, &self.pool).await;
        self.send(request, remote_peers);
    }

    pub fn addr(&self) -> SocketAddr {
        self.udp_socket.local_addr().unwrap()
    }

    pub async fn block(&self, peer: String) -> () {
        let remote_peers = remote::select_by_uid(&self.uid, &self.pool, &peer).await;
        for remote in remote_peers {
            block::add(&self.uid, &self.pool, &remote.addr).await;
            remote::remove_by_uid(&self.uid, &self.pool, &remote.name).await;
        }
    }

    pub fn close(&self) -> () {
        stop_job(&self.udp_socket);
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::HashMap,
        fmt::Debug,
        sync::{Arc, Mutex},
        time::{Duration, SystemTime},
    };

    use async_trait::async_trait;
    use futures::executor::block_on;

    use crate::{
        configuration::Configuration,
        network::{
            events::{Connected, Disconnected, Message},
            Request, Response,
        },
        observer::Observer,
    };

    use super::Peer;

    #[derive(Clone, Default)]
    struct Test {
        connections: Arc<Mutex<Vec<String>>>,
        disconnections: Arc<Mutex<Vec<String>>>,
        messages: Arc<Mutex<HashMap<String, Vec<String>>>>,
    }

    #[async_trait]
    impl Observer for Test {
        async fn on_connected(&mut self, c: Connected) -> Option<Response> {
            let mut connections = self.connections.lock().unwrap();
            connections.push(c.from);
            None
        }
        async fn on_disconnected(&mut self, d: Disconnected) -> Option<Response> {
            let mut disconnections = self.disconnections.lock().unwrap();
            disconnections.push(d.from);
            None
        }
        async fn on_message(&mut self, m: Message) -> Option<Response> {
            let mut messages = self.messages.lock().unwrap();
            match messages.entry(m.from) {
                std::collections::hash_map::Entry::Occupied(mut o) => {
                    let list: &mut Vec<String> = o.get_mut();
                    list.push(String::from_utf8(m.content).unwrap());
                }
                std::collections::hash_map::Entry::Vacant(v) => {
                    v.insert(vec![String::from_utf8(m.content).unwrap()]);
                }
            }
            None
        }
    }

    fn prepare(name: &str, port: u16) -> (Peer, Test) {
        let conf = Configuration::builder()
            .port(port)
            .name(name)
            .share_connections(true)
            .build();
        let test = Test::default();
        let peer = block_on(Peer::new(conf, test.clone()));
        (peer, test)
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
        env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
        let (peer1, test1) = prepare("P1", 9901);
        let (peer2, test2) = prepare("P2", 9902);
        let (peer3, test3) = prepare("P3", 9903);

        let p1_name = String::from("P1");
        let p2_name = String::from("P2");
        let p3_name = String::from("P3");

        // Share connections
        // P1 => P2
        peer1.connect_to(peer2.addr());
        // P3 => P2 ... P2 =(P1)=> P3 ... P3 => P1
        peer3.connect_to(peer2.addr());
        wait_while_condition(&|| {
            test1.connections.lock().unwrap().len() < 2
                || test2.connections.lock().unwrap().len() < 2
                || test3.connections.lock().unwrap().len() < 2
        });
        check(
            vec![p2_name.clone(), p3_name.clone()],
            test1.connections.lock().unwrap().clone(),
        );
        check(
            vec![p1_name.clone(), p3_name.clone()],
            test2.connections.lock().unwrap().clone(),
        );
        check(
            vec![p1_name.clone(), p2_name.clone()],
            test3.connections.lock().unwrap().clone(),
        );

        // Send message to all
        block_on(peer2.send_to_all(Request::new("Hello everybody !")));
        wait_while_condition(&|| {
            test1.messages.lock().unwrap().len() < 1 || test3.messages.lock().unwrap().len() < 1
        });
        check(
            vec![(p2_name.clone(), vec![String::from("Hello everybody !")])],
            Vec::from_iter(test1.messages.lock().unwrap().clone()),
        );
        check(
            vec![(p2_name.clone(), vec![String::from("Hello everybody !")])],
            Vec::from_iter(test3.messages.lock().unwrap().clone()),
        );

        // Send message to peer
        block_on(peer2.send_to(Request::new("What's your name ?"), p1_name.clone()));
        wait_while_condition(&|| {
            test1
                .messages
                .lock()
                .unwrap()
                .get(&p2_name.clone())
                .unwrap()
                .len()
                < 2
        });
        check(
            vec![(
                p2_name.clone(),
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
            vec![p2_name.clone()],
            test1.disconnections.lock().unwrap().clone(),
        );
        check(
            vec![p2_name.clone()],
            test3.disconnections.lock().unwrap().clone(),
        );

        // Disconnection with peer
        block_on(peer1.disconnect_to(p3_name.clone()));
        wait_while_condition(&|| test3.disconnections.lock().unwrap().len() < 2);
        check(
            vec![p1_name.clone(), p2_name.clone()],
            test3.disconnections.lock().unwrap().clone(),
        );
    }
}
