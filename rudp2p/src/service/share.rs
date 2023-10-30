use crate::{
    network::{send, Request, Response},
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
            let req = Request::new_connection(&instance.configuration);
            send(&instance.socket, &req, &remote);
        }
        let res = (None, vec![]);
        log::trace!(
            "ShareService::execute({:?}, {:?}) => {:?}",
            instance,
            request,
            res
        );
        res
    }
}
