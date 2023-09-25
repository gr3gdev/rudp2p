use crate::{
    network::{Request, Response},
    thread::PeerInstance,
};

pub(crate) struct ShareService;

impl ShareService {
    pub(crate) async fn execute(
        instance: &PeerInstance,
        request: &Request,
    ) -> (Option<Response>, Vec<u8>) {
        let remotes = request.to_peers_values();
        for remote in remotes {
            let req = Request::new_connection(&instance.public_key);
            req.send(&instance.socket, &remote, &vec![]);
        }
        (None, vec![])
    }
}
