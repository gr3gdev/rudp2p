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

use crate::{
    dao::{
        self, block,
        part::{self, RequestPart},
        remote, Pool,
    },
    network::{
        events::{Connected, Disconnected, Message},
        request::Type,
        *,
    },
    service::{
        connection::ConnectionService, disconnection::DisconnectionService,
        message::MessageService, share::ShareService,
    },
    utils::multipart::Multipart,
};

static END: &[u8] = "PL3AZE 5T0P".as_bytes();

/// Send a message to stop the thread
pub(crate) fn stop_job(socket: &UdpSocket) {
    let addr = socket.local_addr().unwrap();
    socket
        .send_to(&END.to_vec(), addr)
        .expect("Unable to send end signal");
}

/// Start a thread
pub(crate) async fn start_job<C, D, M>(
    pool: &Pool,
    uid: String,
    socket: UdpSocket,
    private_key: Rsa<Private>,
    public_key: Vec<u8>,
    share_connections: bool,
    on_connected: C,
    on_disconnected: D,
    on_message: M,
) -> ()
where
    C: FnMut(Connected) -> Option<Response> + Send + 'static,
    D: FnMut(Disconnected) -> Option<Response> + Send + 'static,
    M: FnMut(Message) -> Option<Response> + Send + 'static,
{
    let pool = pool.clone();
    // Events
    let on_connected = Arc::new(Mutex::new(Box::new(on_connected)));
    let on_disconnected = Arc::new(Mutex::new(Box::new(on_disconnected)));
    let on_message = Arc::new(Mutex::new(Box::new(on_message)));
    // Thread
    thread::spawn(move || {
        let mut buf = [0; 2048];
        block_on(dao::thread::update(&pool, true));
        info!("Peer {} started.", uid);
        loop {
            debug!("Peer {uid} wait message...");
            match socket.recv_from(&mut buf) {
                Ok((number_of_bytes, addr)) => {
                    if let ControlFlow::Break(reason) = block_on(process_message(
                        &pool,
                        addr,
                        buf,
                        number_of_bytes,
                        &uid,
                        &socket,
                        &public_key,
                        &private_key,
                        share_connections,
                        &on_connected,
                        &on_disconnected,
                        &on_message,
                    )) {
                        debug!("Peer {uid} - break : {reason}");
                        break;
                    }
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    // wait until network socket is ready
                    debug!("Peer {uid} - socket is not ready !");
                }
                Err(e) => error!("{e}"),
            }
        }
        block_on(dao::thread::update(&pool, false));
        info!("Peer {} stopped.", uid);
    });
}

async fn process_message<C, D, M>(
    pool: &Pool,
    addr: SocketAddr,
    buf: [u8; 2048],
    number_of_bytes: usize,
    uid: &String,
    socket: &UdpSocket,
    public_key: &Vec<u8>,
    private_key: &Rsa<Private>,
    share_connections: bool,
    on_connected: &Arc<Mutex<Box<C>>>,
    on_disconnected: &Arc<Mutex<Box<D>>>,
    on_message: &Arc<Mutex<Box<M>>>,
) -> ControlFlow<String>
where
    C: FnMut(Connected) -> Option<Response> + Send + 'static,
    D: FnMut(Disconnected) -> Option<Response> + Send + 'static,
    M: FnMut(Message) -> Option<Response> + Send + 'static,
{
    let blocked = block::select_all(pool).await;
    // Only if address is not blocked
    if !blocked.contains(&addr) {
        let data = buf[..number_of_bytes].to_vec();
        if data == END {
            // Receive END
            let peers = remote::select_all(pool).await;
            for remote in peers {
                let addr = remote.addr;
                let request = Request::new_disconnection(uid.clone());
                request.send(socket, &addr, public_key);
            }
            return ControlFlow::Break(String::from("Server stopped"));
        } else {
            let part = RequestPart::parse(data);
            debug!("Peer {uid} receive {:?} from {}", part, addr);
            let (res, pk) = process_response(
                pool,
                (uid, public_key),
                socket,
                part,
                addr,
                private_key,
                None,
                share_connections,
                Arc::clone(on_connected),
                Arc::clone(on_disconnected),
                Arc::clone(on_message),
            )
            .await;
            if let Some(response) = res {
                let request = response.to_request(uid.clone());
                request.send(socket, &addr, &pk);
            }
        }
    } else {
        debug!("Peer {uid} - request is blocked !");
    }
    ControlFlow::Continue(())
}

async fn process_response<C, D, M>(
    pool: &Pool,
    peer: (&String, &Vec<u8>),
    socket: &UdpSocket,
    part: RequestPart,
    addr: SocketAddr,
    private_key: &Rsa<Private>,
    remote_public_key: Option<Vec<u8>>,
    share_connections: bool,
    on_connected: Arc<Mutex<Box<C>>>,
    on_disconnected: Arc<Mutex<Box<D>>>,
    on_message: Arc<Mutex<Box<M>>>,
) -> (Option<Response>, Vec<u8>)
where
    C: FnMut(Connected) -> Option<Response> + Send + 'static,
    D: FnMut(Disconnected) -> Option<Response> + Send + 'static,
    M: FnMut(Message) -> Option<Response> + Send + 'static,
{
    let part_uid = part.uid.clone();
    let request_type = part.request_type.clone();
    let total = part.clone().total;
    let mut parts = part::select_by_uid(pool, &part_uid).await;
    part::add(pool, &part).await;
    parts.push(part);
    let connected_peers = remote::select_all(pool).await;
    if let Some(request) = Multipart::merge(parts.clone(), request_type, total, private_key) {
        // Request is merged and completed
        part::remove_by_uid(pool, &part_uid).await;
        match request.request_type {
            Type::Connection => {
                ConnectionService::execute(
                    pool,
                    socket,
                    &request,
                    &peer.0,
                    &peer.1,
                    &addr,
                    &connected_peers,
                    share_connections,
                    on_connected,
                )
                .await
            }
            Type::Disconnection => {
                DisconnectionService::execute(
                    pool,
                    socket,
                    &request,
                    &peer.0,
                    &addr,
                    on_disconnected,
                )
                .await
            }
            Type::Message => {
                MessageService::execute(&request, &peer.0, &addr, remote_public_key, on_message)
                    .await
            }
            Type::ShareConnection => {
                ShareService::execute(socket, &request, &peer.0, &peer.1).await
            }
        }
    } else {
        debug!("Peer {} - incomplete request...", peer.0);
        (None, vec![])
    }
}
