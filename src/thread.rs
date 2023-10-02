use crate::{
    configuration::Configuration,
    dao::{
        self, block,
        part::{self, RequestPart},
        remote, Pool,
    },
    network::{request::Type, *},
    observer::Observer,
    service::{
        connection::ConnectionService, disconnection::DisconnectionService,
        message::MessageService, share::ShareService,
    },
    utils::multipart::Multipart,
};
use futures::executor::block_on;
use log::{debug, error, info};
use openssl::{pkey::Private, rsa::Rsa};
use std::{
    io,
    net::{SocketAddr, UdpSocket},
    ops::ControlFlow,
    sync::{Arc, Mutex},
    thread,
};

static END: &[u8] = "PL3AZE 5T0P".as_bytes();

pub(crate) struct PeerInstance {
    pub(crate) pool: Arc<Pool>,
    private_key: Rsa<Private>,
    pub(crate) public_key: Vec<u8>,
    pub(crate) socket: UdpSocket,
    pub(crate) configuration: Configuration,
}

impl Clone for PeerInstance {
    fn clone(&self) -> Self {
        Self {
            pool: self.pool.clone(),
            private_key: self.private_key.clone(),
            public_key: self.public_key.clone(),
            socket: self.socket.try_clone().unwrap(),
            configuration: self.configuration.clone(),
        }
    }
}

impl PeerInstance {
    pub(crate) async fn new(
        configuration: &Configuration,
        private_key: Rsa<Private>,
        socket: &UdpSocket,
    ) -> Self {
        // Public key PEM
        let public_key = private_key
            .public_key_to_pem()
            .or_else(|e| {
                error!("Unable to generate public key : {e}");
                Err("Unable to generate public key")
            })
            .unwrap();

        // Init database
        let pool = dao::init(configuration).await;

        Self {
            pool: Arc::new(pool),
            private_key,
            public_key,
            socket: socket.try_clone().unwrap(),
            configuration: configuration.clone(),
        }
    }
}

/// Send a message to stop the thread
pub(crate) fn stop_job(socket: &UdpSocket) {
    let addr = socket.local_addr().unwrap();
    socket
        .send_to(&END.to_vec(), addr)
        .expect("Unable to send end signal");
}

/// Start a thread
pub(crate) async fn start_socket_job<O>(
    configuration: &Configuration,
    socket: &UdpSocket,
    observer: O,
) -> PeerInstance
where
    O: Observer,
{
    // Generate SSL keys
    let rsa = Rsa::generate(2048)
        .or_else(|e| {
            error!("Unable to generate SSL keys : {e}");
            Err("Unable to generate SSL keys")
        })
        .unwrap();

    // Init the peer instance for internal threads
    let instance = PeerInstance::new(configuration, rsa, &socket).await;
    let observer = Arc::new(Mutex::new(observer));
    let thread_instance = instance.clone();

    // Thread
    thread::spawn(move || {
        let mut buf = [0; 2048];
        block_on(async {
            if dao::thread::update(&thread_instance.pool, true).await < 1 {
                error!("[DAO] Unable to update thread status");
            }
        });
        info!("Peer started on port {}.", instance.configuration.port);
        loop {
            debug!("Waiting message...");
            match thread_instance.socket.recv_from(&mut buf) {
                Ok((number_of_bytes, addr)) => {
                    let (control_flow, part) = block_on(save_part_or_break(
                        &thread_instance,
                        addr,
                        buf,
                        number_of_bytes,
                    ));
                    if let ControlFlow::Break(reason) = control_flow {
                        debug!("{reason}");
                        break;
                    } else if let Some(part) = part {
                        block_on(process_complete_parts(
                            &thread_instance,
                            &part.uid,
                            part.total,
                            Arc::clone(&observer),
                        ));
                    }
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    // wait until network socket is ready
                    debug!("Socket is not ready !");
                }
                Err(e) => error!("{e}"),
            }
        }
        block_on(async {
            if dao::thread::update(&thread_instance.pool, false).await < 1 {
                error!("[DAO] Unable to update thread status");
            }
        });
        info!("Peer stopped on port {}.", instance.configuration.port);
    });
    instance
}

async fn save_part_or_break(
    instance: &PeerInstance,
    addr: SocketAddr,
    buf: [u8; 2048],
    number_of_bytes: usize,
) -> (ControlFlow<String>, Option<RequestPart>) {
    let blocked = block::select_all(&instance.pool).await;
    // Only if address is not blocked
    if !blocked.contains(&addr) {
        let data = buf[..number_of_bytes].to_vec();
        if data == END {
            // Receive END
            let peers = remote::select_all(&instance.pool).await;
            for remote in peers {
                let addr = remote.addr;
                let request = Request::new_disconnection();
                request.send(&instance.socket, &addr, &instance.public_key);
            }
            (ControlFlow::Break(String::from("Receive END")), None)
        } else {
            // Save request part
            let part = RequestPart::parse(data, addr);
            if part::add(&instance.pool, &part).await < 1 {
                error!("[DAO] Unable to save request part {}", part.uid);
                (ControlFlow::Continue(()), None)
            } else {
                debug!("Receive {:?}", part);
                (ControlFlow::Continue(()), Some(part))
            }
        }
    } else {
        debug!("Request is blocked !");
        (ControlFlow::Continue(()), None)
    }
}

fn is_complete_parts(parts: &Vec<RequestPart>, total: usize) -> bool {
    let total_parts: usize = parts.iter().map(|r| r.content_size).sum();
    total_parts == total
}

async fn process_complete_parts<O>(
    instance: &PeerInstance,
    part_uid: &String,
    total: usize,
    observer: Arc<Mutex<O>>,
) -> ()
where
    O: Observer,
{
    let parts = part::select_by_uid(&instance.pool, part_uid).await;
    // Check parts are completed
    if is_complete_parts(&parts, total) {
        let instance = instance.clone();
        let part_uid = part_uid.clone();
        // Thread
        thread::spawn(move || {
            // Request is merged and completed : clean table
            block_on(async {
                if part::remove_by_uid(&instance.pool, &part_uid).await < 1 {
                    log::error!("[DAO] Unable to remove request part {}", part_uid);
                }
                // Merge parts into Request
                let (request, remote_addr) = Multipart::merge(&parts, &instance.private_key);
                // Process the request
                process_request(&instance, &request, &remote_addr, Arc::clone(&observer)).await;
            });
        });
    }
}

async fn process_request<O>(
    instance: &PeerInstance,
    request: &Request,
    remote_addr: &SocketAddr,
    observer: Arc<Mutex<O>>,
) -> ()
where
    O: Observer,
{
    let observer = Arc::clone(&observer);
    match request.request_type {
        Type::Connection => {
            process_response(
                &instance,
                &remote_addr,
                ConnectionService::execute(&instance, &request, &remote_addr, observer).await,
            )
            .await
        }
        Type::Disconnection => {
            process_response(
                &instance,
                &remote_addr,
                DisconnectionService::execute(&instance, &remote_addr, observer).await,
            )
            .await
        }
        Type::Message => {
            process_response(
                &instance,
                &remote_addr,
                MessageService::execute(&instance, &request, &remote_addr, observer).await,
            )
            .await
        }
        Type::ShareConnection => {
            process_response(
                &instance,
                &remote_addr,
                ShareService::execute(&instance, &request).await,
            )
            .await
        }
    }
}

async fn process_response(
    instance: &PeerInstance,
    addr: &SocketAddr,
    res: (Option<Response>, Vec<u8>),
) -> () {
    if let (Some(response), pk) = res {
        let request = response.to_request();
        request.send(&instance.socket, addr, &pk);
    }
}
