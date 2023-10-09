use crate::{
    configuration::Configuration,
    dao,
    network::{request::Type, *},
    observer::Observer,
    service::{
        connection::ConnectionService, disconnection::DisconnectionService,
        message::MessageService, share::ShareService,
    },
    utils::{multipart::Multipart, unwrap::unwrap_result},
};
use futures::executor::block_on;
use std::{
    fmt::Debug,
    io,
    net::{SocketAddr, UdpSocket},
    ops::ControlFlow,
    sync::{Arc, Mutex},
    thread,
};

static END: &[u8] = "PL3AZE 5T0P".as_bytes();

pub(crate) struct PeerInstance {
    pub(crate) pool: Arc<dao::Pool>,
    pub(crate) socket: UdpSocket,
    pub(crate) configuration: Configuration,
}

impl Debug for PeerInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PeerInstance")
            .field("pool", &self.pool)
            .field("socket", &self.socket)
            .field("configuration", &self.configuration)
            .finish()
    }
}

impl Clone for PeerInstance {
    fn clone(&self) -> Self {
        Self {
            pool: self.pool.clone(),
            socket: unwrap_result(self.socket.try_clone(), "Unable to clone socket"),
            configuration: self.configuration.clone(),
        }
    }
}

impl PeerInstance {
    pub(crate) async fn new(configuration: &Configuration, socket: &UdpSocket) -> Self {
        // Init database
        let pool = dao::init(configuration).await;

        let instance = Self {
            pool: Arc::new(pool),
            socket: unwrap_result(socket.try_clone(), "Unable to clone socket"),
            configuration: configuration.clone(),
        };
        log::trace!(
            "PeerInstance::new({:?}, {:?}) => {:?}",
            configuration,
            socket,
            instance
        );
        instance
    }
}

/// Send a message to stop the thread
pub(crate) fn stop_job(socket: &UdpSocket) -> () {
    log::trace!("stop_job({:?})", socket);
    let addr = unwrap_result(socket.local_addr(), "Unable to get the local address");
    unwrap_result(
        socket.send_to(&END.to_vec(), addr),
        "Unable to send end signal",
    );
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
    // Init the peer instance for internal threads
    let instance = PeerInstance::new(configuration, &socket).await;
    let observer = Arc::new(Mutex::new(observer));
    let thread_instance = instance.clone();

    // Thread
    thread::spawn(move || {
        let mut buf = [0; 2048];
        block_on(async {
            if dao::thread::update(&thread_instance.pool, true).await < 1 {
                log::error!("[DAO] Unable to update thread status");
            }
        });
        log::info!("Peer started on port {}.", instance.configuration.port);
        loop {
            log::debug!("Waiting message...");
            match thread_instance.socket.recv_from(&mut buf) {
                Ok((number_of_bytes, addr)) => {
                    let (control_flow, part) = block_on(save_part_or_break(
                        &thread_instance,
                        addr,
                        buf,
                        number_of_bytes,
                    ));
                    if let ControlFlow::Break(reason) = control_flow {
                        log::debug!("{reason}");
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
                    log::debug!("Socket is not ready !");
                }
                Err(e) => log::error!("{e}"),
            }
        }
        block_on(async {
            if dao::thread::update(&thread_instance.pool, false).await < 1 {
                log::error!("[DAO] Unable to update thread status");
            }
        });
        log::info!("Peer stopped on port {}.", instance.configuration.port);
    });
    log::trace!(
        "start_socket_job({:?}, {:?}, observer) => {:?}",
        configuration,
        socket,
        instance
    );
    instance
}

async fn save_part_or_break(
    instance: &PeerInstance,
    addr: SocketAddr,
    buf: [u8; 2048],
    number_of_bytes: usize,
) -> (ControlFlow<String>, Option<dao::part::RequestPart>) {
    let blocked = dao::block::select_all(&instance.pool).await;
    // Only if address is not blocked
    let res = if !blocked.contains(&addr) {
        let data = buf[..number_of_bytes].to_vec();
        if data == END {
            // Receive END
            let peers = dao::remote::select_all(&instance.pool).await;
            for remote in peers {
                send_disonnection(instance, remote.addr);
            }
            (ControlFlow::Break(String::from("Receive END")), None)
        } else {
            // Save request part
            let part = dao::part::RequestPart::parse(data, addr);
            if dao::part::add(&instance.pool, &part).await < 1 {
                log::error!("[DAO] Unable to save request part {}", part.uid);
                (ControlFlow::Continue(()), None)
            } else {
                log::debug!("Receive {:?}", part);
                (ControlFlow::Continue(()), Some(part))
            }
        }
    } else {
        log::debug!("Request is blocked !");
        (ControlFlow::Continue(()), None)
    };
    log::trace!(
        "save_part_or_break({:?}, {addr}, {}, {number_of_bytes}) => {:?}",
        instance,
        buf.len(),
        res
    );
    res
}

#[cfg(not(feature = "ssl"))]
fn send_disonnection(instance: &PeerInstance, addr: SocketAddr) {
    let request = Request::new_disconnection();
    request.send(&instance.socket, &addr);
}

#[cfg(feature = "ssl")]
fn send_disonnection(instance: &PeerInstance, addr: SocketAddr) {
    let request = Request::new_disconnection();
    request.send(
        &instance.socket,
        &addr,
        &instance.configuration.ssl.public_key,
    );
}

fn is_complete_parts(parts: &Vec<dao::part::RequestPart>, total: usize) -> bool {
    let total_parts: usize = parts.iter().map(|r| r.content_size).sum();
    let res = total_parts == total;
    log::trace!("is_complete_parts({:?}, {total}) => {res}", parts);
    res
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
    log::trace!(
        "process_complete_parts({:?}, {part_uid}, {total}, observer)",
        instance
    );
    let parts = dao::part::select_by_uid(&instance.pool, part_uid).await;
    // Check parts are completed
    if is_complete_parts(&parts, total) {
        let instance = instance.clone();
        let part_uid = part_uid.clone();
        // Thread
        thread::spawn(move || {
            // Request is merged and completed : clean table
            block_on(async {
                if dao::part::remove_by_uid(&instance.pool, &part_uid).await < 1 {
                    log::error!("[DAO] Unable to remove request part {}", part_uid);
                }
                // Merge parts into Request
                let (request, remote_addr) = Multipart::merge(&parts, &instance.configuration);
                // Process the request
                process_request(&instance, &request, &remote_addr, Arc::clone(&observer)).await;
            });
        });
    } else {
        log::debug!("Imcomplete request {:?}", parts);
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
    log::trace!(
        "process_request({:?}, {:?}, {remote_addr}, observer)",
        instance,
        request
    );
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

#[cfg(not(feature = "ssl"))]
async fn process_response(
    instance: &PeerInstance,
    addr: &SocketAddr,
    res: (Option<Response>, Vec<u8>),
) -> () {
    log::trace!("process_response({:?}, {addr}, {:?})", instance, res);
    if let (Some(response), _) = res {
        let request = response.to_request();
        request.send(&instance.socket, addr);
    }
}

#[cfg(feature = "ssl")]
async fn process_response(
    instance: &PeerInstance,
    addr: &SocketAddr,
    res: (Option<Response>, Vec<u8>),
) -> () {
    log::trace!("process_response({:?}, {addr}, {:?})", instance, res);
    if let (Some(response), pk) = res {
        let request = response.to_request();
        request.send(&instance.socket, addr, &pk);
    }
}
