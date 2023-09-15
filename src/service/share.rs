use std::net::UdpSocket;

use crate::network::{Request, Response};

pub(crate) struct ShareService;

impl ShareService {
    pub(crate) async fn execute(
        socket: &UdpSocket,
        request: &Request,
        peer_uid: &String,
        public_key: &Vec<u8>,
    ) -> (Option<Response>, Vec<u8>) {
        let remotes = request.to_peers_values();
        for remote in remotes {
            let req = Request::new_connection(peer_uid.clone(), public_key.clone());
            req.send(socket, &remote, &vec![]);
        }
        (None, vec![])
    }
}
