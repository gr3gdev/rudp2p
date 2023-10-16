use crate::{network::request::RequestPart, peer::RemotePeer};
use async_trait::async_trait;
use std::{
    collections::HashMap,
    fmt::Debug,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

#[async_trait]
pub trait PeerDao: Debug + Send + 'static {
    async fn init(&mut self) -> ();

    #[cfg(feature = "ssl")]
    async fn add_remote(&self, address: &SocketAddr, public_key: &Vec<u8>) -> RemotePeer;
    #[cfg(not(feature = "ssl"))]
    async fn add_remote(&self, address: &SocketAddr) -> RemotePeer;

    async fn remove_remote(&self, remote: &RemotePeer) -> usize;

    async fn find_remotes_by_address(&self, address: &SocketAddr) -> Vec<RemotePeer>;

    async fn find_all_remotes(&self) -> Vec<RemotePeer>;

    async fn add_request_part(&self, part: &RequestPart) -> usize;

    async fn remove_request_part_by_uid(&self, uid: &String) -> usize;

    async fn find_requests_part_by_uid(&self, uid: &String) -> Vec<RequestPart>;

    async fn block(&self, address: &SocketAddr) -> usize;

    async fn unblock(&self, address: &SocketAddr) -> usize;

    async fn find_all_block(&self) -> Vec<SocketAddr>;

    async fn update_status(&self, value: bool) -> ();

    async fn find_status(&self) -> bool;
}

#[derive(Debug, Default, Clone)]
pub struct InMemoryDao {
    remote_peer_dao: RemotePeerDao,
    request_part_dao: RequestPartDao,
    bloc_dao: BlockDao,
    status_dao: StatusDao,
}

#[derive(Debug, Default, Clone)]
pub(crate) struct RemotePeerDao {
    remotes: Arc<Mutex<Vec<RemotePeer>>>,
}

#[derive(Debug, Default, Clone)]
pub(crate) struct RequestPartDao {
    parts: Arc<Mutex<HashMap<String, Vec<RequestPart>>>>,
}

#[derive(Debug, Default, Clone)]
pub(crate) struct BlockDao {
    black_list: Arc<Mutex<Vec<SocketAddr>>>,
}

#[derive(Debug, Default, Clone)]
pub(crate) struct StatusDao {
    status: Arc<Mutex<bool>>,
}

impl RemotePeerDao {
    #[cfg(feature = "ssl")]
    fn add(&self, address: &SocketAddr, public_key: &Vec<u8>) -> RemotePeer {
        let mut remotes = self.remotes.lock().unwrap();
        let remote = RemotePeer {
            addr: address.clone(),
            public_key: public_key.clone(),
        };
        remotes.push(remote.clone());
        remote
    }
    #[cfg(not(feature = "ssl"))]
    fn add(&self, address: &SocketAddr) -> RemotePeer {
        let mut remotes = self.remotes.lock().unwrap();
        let remote = RemotePeer {
            addr: address.clone(),
        };
        remotes.push(remote.clone());
        remote
    }

    fn remove(&self, remote: &RemotePeer) -> usize {
        let mut remotes = self.remotes.lock().unwrap();
        let index = remotes.iter().position(|r| r == remote).unwrap();
        remotes.remove(index);
        1
    }

    fn find_by_address(&self, address: &SocketAddr) -> Vec<RemotePeer> {
        self.remotes
            .lock()
            .unwrap()
            .iter()
            .filter(|r| &r.addr == address)
            .map(RemotePeer::clone)
            .collect()
    }

    fn find_all(&self) -> Vec<RemotePeer> {
        self.remotes.lock().unwrap().clone()
    }
}

impl RequestPartDao {
    fn add(&self, part: &RequestPart) -> usize {
        let mut parts = self.parts.lock().unwrap();
        match parts.entry(part.uid.clone()) {
            std::collections::hash_map::Entry::Occupied(mut o) => {
                let list = o.get_mut();
                list.push(part.clone());
            }
            std::collections::hash_map::Entry::Vacant(v) => {
                v.insert(vec![part.clone()]);
            }
        }
        1
    }

    fn remove_by_uid(&self, uid: &String) -> usize {
        let mut parts = self.parts.lock().unwrap();
        parts.remove(uid);
        1
    }

    fn find_by_uid(&self, uid: &String) -> Vec<RequestPart> {
        self.parts.lock().unwrap().get(uid).unwrap().clone()
    }
}

impl BlockDao {
    fn add(&self, address: &SocketAddr) -> usize {
        let mut list = self.black_list.lock().unwrap();
        list.push(address.clone());
        1
    }

    fn remove(&self, address: &SocketAddr) -> usize {
        let mut list = self.black_list.lock().unwrap();
        let index = list.iter().position(|a| a == address).unwrap();
        list.remove(index);
        1
    }

    fn find_all(&self) -> Vec<SocketAddr> {
        self.black_list.lock().unwrap().clone()
    }
}

impl StatusDao {
    fn update(&self, value: bool) -> () {
        let mut status = self.status.lock().unwrap();
        *status = value;
    }

    fn find(&self) -> bool {
        self.status.lock().unwrap().clone()
    }
}

#[async_trait]
impl PeerDao for InMemoryDao {
    async fn init(&mut self) -> () {
        self.remote_peer_dao = RemotePeerDao::default();
        self.request_part_dao = RequestPartDao::default();
        self.bloc_dao = BlockDao::default();
        self.status_dao = StatusDao::default();
    }

    #[cfg(feature = "ssl")]
    async fn add_remote(&self, address: &SocketAddr, public_key: &Vec<u8>) -> RemotePeer {
        self.remote_peer_dao.add(address, public_key)
    }
    #[cfg(not(feature = "ssl"))]
    async fn add_remote(&self, address: &SocketAddr) -> RemotePeer {
        self.remote_peer_dao.add(address)
    }

    async fn remove_remote(&self, remote: &RemotePeer) -> usize {
        self.remote_peer_dao.remove(remote)
    }

    async fn find_remotes_by_address(&self, address: &SocketAddr) -> Vec<RemotePeer> {
        self.remote_peer_dao.find_by_address(address)
    }

    async fn find_all_remotes(&self) -> Vec<RemotePeer> {
        self.remote_peer_dao.find_all()
    }

    async fn add_request_part(&self, part: &RequestPart) -> usize {
        self.request_part_dao.add(part)
    }

    async fn remove_request_part_by_uid(&self, uid: &String) -> usize {
        self.request_part_dao.remove_by_uid(uid)
    }

    async fn find_requests_part_by_uid(&self, uid: &String) -> Vec<RequestPart> {
        self.request_part_dao.find_by_uid(uid)
    }

    async fn block(&self, address: &SocketAddr) -> usize {
        self.bloc_dao.add(address)
    }

    async fn unblock(&self, address: &SocketAddr) -> usize {
        self.bloc_dao.remove(address)
    }

    async fn find_all_block(&self) -> Vec<SocketAddr> {
        self.bloc_dao.find_all()
    }

    async fn update_status(&self, value: bool) -> () {
        self.status_dao.update(value)
    }

    async fn find_status(&self) -> bool {
        self.status_dao.find()
    }
}
