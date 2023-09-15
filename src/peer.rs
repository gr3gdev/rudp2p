use log::{debug, error, info};
use openssl::rsa::Rsa;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use std::{
    fmt::Debug,
    net::{IpAddr, SocketAddr, UdpSocket},
};

use crate::{
    configuration::Configuration,
    dao::{
        block, create_or_upgrade_db,
        remote::{self, RemotePeer},
        thread,
    },
    network::{events::*, *},
    thread::{start_job, stop_job},
    utils::generate_uid,
};

/// Build an uid with timestamp (in nano seconds)
fn build_uid(uid: Option<String>) -> String {
    uid.unwrap_or_else(|| generate_uid("P"))
}

/// # Peer
///
/// Example of a connection
/// ```
/// use rudp2plib::{configuration::*, network::*, peer::*};
///
/// async fn example() {
///     env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
///

///     let peer1 = Peer::new(
///         Configuration::builder().port(9001).name("Peer1").build(),
///         |c| Some(Response::text("Hello I am Peer1 !")),
///         |d| Some(Response::text("Goodbye !")),
///         |m| None,
///     ).await;
///
///     let peer2 = Peer::new(
///         Configuration::builder().port(9002).name("Peer2").build(),
///         |c| Some(Response::text("Hello I am Peer2 !")),
///         |d| Some(Response::text("Goodbye !")),
///         |m| None,
///     ).await;
///
///     peer1.connect_to(peer2.addr());
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
    pub async fn new<C, D, M>(
        configuration: Configuration,
        on_connected: C,
        on_disconnected: D,
        on_message: M,
    ) -> Peer
    where
        C: FnMut(Connected) -> Option<Response> + Send + 'static,
        D: FnMut(Disconnected) -> Option<Response> + Send + 'static,
        M: FnMut(Message) -> Option<Response> + Send + 'static,
    {
        // Init database
        let manager = SqliteConnectionManager::memory();
        let pool = Pool::new(manager).expect("Unable to initialize pool");
        create_or_upgrade_db(&pool).await;

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

        // Generate SSL keys
        let rsa = Rsa::generate(2048)
            .or_else(|e| {
                error!("Unable to generate SSL keys : {e}");
                Err("Unable to generate SSL keys")
            })
            .unwrap();

        // Public key PEM
        let pk = rsa
            .public_key_to_pem()
            .or_else(|e| {
                error!("Unable to generate public key : {e}");
                Err("Unable to generate public key")
            })
            .unwrap();

        let uid = build_uid(configuration.name);

        // Start thread for send and receive messages
        start_job(
            &pool,
            uid.clone(),
            socket.try_clone().expect("Unable to clone socket"),
            rsa,
            pk.clone(),
            configuration.share_connections,
            on_connected,
            on_disconnected,
            on_message,
        )
        .await;

        // Return Peer
        Peer {
            uid,
            udp_socket: socket,
            public_key_pem: pk,
            pool,
        }
    }

    pub async fn is_alive(&self) -> bool {
        thread::status(&self.pool).await
    }

    pub fn connect_to(&self, addr: SocketAddr) -> () {
        info!("Peer {} connect to {}", self.uid, addr);
        let request = Request::new_connection(self.uid.clone(), self.public_key_pem.clone());
        request.send(&self.udp_socket, &addr, &vec![]);
    }

    fn send(&self, request: Request, remote_peers: Vec<RemotePeer>) -> () {
        for remote in remote_peers {
            debug!("Peer {} send {:?} to {:?}", self.uid, request, remote);
            request.send(&self.udp_socket, &remote.addr, &remote.public_key);
        }
    }

    pub async fn send_to(&self, request: Request, peer: String) -> () {
        let request = Request::new_message(self.uid.clone(), request.content);
        let remote_peers = remote::select_by_uid(&self.pool, &peer).await;
        self.send(request, remote_peers);
    }

    pub async fn send_to_all(&self, request: Request) -> () {
        let request = Request::new_message(self.uid.clone(), request.content);
        let remote_peers = remote::select_all(&self.pool).await;
        self.send(request, remote_peers);
    }

    pub async fn disconnect_to(&self, peer: String) -> () {
        let request = Request::new_disconnection(self.uid.clone());
        let remote_peers = remote::select_by_uid(&self.pool, &peer).await;
        self.send(request, remote_peers);
    }

    pub async fn disconnect_to_all(&self) -> () {
        let request = Request::new_disconnection(self.uid.clone());
        let remote_peers = remote::select_all(&self.pool).await;
        self.send(request, remote_peers);
    }

    pub fn addr(&self) -> SocketAddr {
        self.udp_socket.local_addr().unwrap()
    }

    pub async fn block(&self, peer: String) -> () {
        let remote_peers = remote::select_by_uid(&self.pool, &peer).await;
        for remote in remote_peers {
            block::add(&self.pool, &remote.addr).await;
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

    use futures::executor::block_on;

    use crate::{configuration::Configuration, network::Request};

    use super::Peer;

    fn prepare(
        name: &str,
        port: u16,
    ) -> (
        Peer,
        Arc<Mutex<Vec<String>>>,
        Arc<Mutex<Vec<String>>>,
        Arc<Mutex<HashMap<String, Vec<String>>>>,
    ) {
        let peer_connections = Arc::new(Mutex::new(Vec::new()));
        let peer_disconnections = Arc::new(Mutex::new(Vec::new()));
        let peer_messages = Arc::new(Mutex::new(HashMap::new()));
        let conf = Configuration::builder()
            .port(port)
            .name(name)
            .share_connections(true)
            .build();
        let share_connections = Arc::clone(&peer_connections);
        let share_disconnections = Arc::clone(&peer_disconnections);
        let share_messages = Arc::clone(&peer_messages);
        let peer = block_on(Peer::new(
            conf,
            move |c| {
                let mut connections = share_connections.lock().unwrap();
                connections.push(c.from);
                None
            },
            move |d| {
                let mut disconnections = share_disconnections.lock().unwrap();
                disconnections.push(d.from);
                None
            },
            move |m| {
                let mut messages = share_messages.lock().unwrap();
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
            },
        ));
        (peer, peer_connections, peer_disconnections, peer_messages)
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
        let (peer1, peer1_connections, peer1_disconnections, peer1_messages) = prepare("P1", 9901);
        let (peer2, peer2_connections, _, _) = prepare("P2", 9902);
        let (peer3, peer3_connections, peer3_disconnections, peer3_messages) = prepare("P3", 9903);

        let p1_name = String::from("P1");
        let p2_name = String::from("P2");
        let p3_name = String::from("P3");

        // Share connections
        // P1 => P2
        peer1.connect_to(peer2.addr());
        // P3 => P2 ... P2 =(P1)=> P3 ... P3 => P1
        peer3.connect_to(peer2.addr());
        wait_while_condition(&|| {
            peer1_connections.lock().unwrap().len() < 2
                || peer2_connections.lock().unwrap().len() < 2
                || peer3_connections.lock().unwrap().len() < 2
        });
        check(
            vec![p2_name.clone(), p3_name.clone()],
            peer1_connections.lock().unwrap().clone(),
        );
        check(
            vec![p1_name.clone(), p3_name.clone()],
            peer2_connections.lock().unwrap().clone(),
        );
        check(
            vec![p1_name.clone(), p2_name.clone()],
            peer3_connections.lock().unwrap().clone(),
        );

        // Send message to all
        block_on(peer2.send_to_all(Request::new("Hello everybody !")));
        wait_while_condition(&|| {
            peer1_messages.lock().unwrap().len() < 1 || peer3_messages.lock().unwrap().len() < 1
        });
        check(
            vec![(p2_name.clone(), vec![String::from("Hello everybody !")])],
            Vec::from_iter(peer1_messages.lock().unwrap().clone()),
        );
        check(
            vec![(p2_name.clone(), vec![String::from("Hello everybody !")])],
            Vec::from_iter(peer3_messages.lock().unwrap().clone()),
        );

        // Send message to peer
        block_on(peer2.send_to(Request::new("What's your name ?"), p1_name.clone()));
        wait_while_condition(&|| {
            peer1_messages
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
            Vec::from_iter(peer1_messages.lock().unwrap().clone()),
        );

        // Disconnection with all
        block_on(peer2.disconnect_to_all());
        wait_while_condition(&|| {
            peer1_disconnections.lock().unwrap().len() < 1
                || peer3_disconnections.lock().unwrap().len() < 1
        });
        check(
            vec![p2_name.clone()],
            peer1_disconnections.lock().unwrap().clone(),
        );
        check(
            vec![p2_name.clone()],
            peer3_disconnections.lock().unwrap().clone(),
        );

        // Disconnection with peer
        block_on(peer1.disconnect_to(p3_name.clone()));
        wait_while_condition(&|| peer3_disconnections.lock().unwrap().len() < 2);
        check(
            vec![p1_name.clone(), p2_name.clone()],
            peer3_disconnections.lock().unwrap().clone(),
        );
    }
}
