use std::{fmt::Debug, net::SocketAddr};

use super::*;

use crate::network::events::Connected;

use log::{debug, error};

#[derive(Clone)]
pub(crate) struct RemotePeer {
    pub(crate) name: String,
    pub(crate) addr: SocketAddr,
    pub(crate) public_key: Vec<u8>,
}

impl Debug for RemotePeer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RemotePeer")
            .field("name", &self.name)
            .field("addr", &self.addr)
            .finish()
    }
}

impl RemotePeer {
    pub(crate) fn new(connected: Connected) -> Self {
        Self {
            name: connected.from,
            addr: connected.address,
            public_key: connected.public_key,
        }
    }
}

pub(crate) async fn create_or_upgrade(connection: &Connection) {
    connection
        .execute(
            "
    CREATE TABLE IF NOT EXISTS remote_peer (
        uid TEXT PRIMARY KEY,
        address TEXT,
        public_key TEXT
    )",
            [],
        )
        .expect("Unable to create table 'remote_peer'");
}

fn mapper(row: &rusqlite::Row<'_>) -> RemotePeer {
    let name = row.get(0).expect("Unable to read 'uid'");
    let address: String = row.get(1).expect("Unable to read 'address'");
    let public_key: String = row.get(2).expect("Unable to read 'public_key'");
    RemotePeer {
        name,
        addr: address.parse().expect("Unable to parse address"),
        public_key: public_key.as_bytes().to_vec(),
    }
}

pub(crate) async fn select_all(pool: &Pool) -> Vec<RemotePeer> {
    let connection = get_connection(pool).await;
    let mut statement = connection
        .prepare("SELECT uid, address, public_key FROM remote_peer")
        .expect("Unable to prepare query : select_all");
    statement
        .query_map([], |row| {
            let remote = mapper(row);
            debug!("select_all() = {:?}", remote);
            Ok(remote)
        })
        .and_then(Iterator::collect)
        .unwrap_or_else(|e| {
            error!("{e}");
            vec![]
        })
}

pub(crate) async fn select_by_uid(pool: &Pool, uid: &String) -> Vec<RemotePeer> {
    let connection = get_connection(pool).await;
    let mut statement = connection
        .prepare("SELECT uid, address, public_key FROM remote_peer WHERE uid = ?1")
        .expect("Unable to prepare query : select_by_uid");
    statement
        .query_map([uid], |row| {
            let remote = mapper(row);
            debug!("select_by_uid({}) = {:?}", uid, remote);
            Ok(remote)
        })
        .and_then(Iterator::collect)
        .unwrap_or_else(|e| {
            error!("{e}");
            vec![]
        })
}

pub(crate) async fn add(pool: &Pool, remote: &RemotePeer) -> usize {
    let address = remote.addr.to_string();
    let public_key = String::from_utf8(remote.public_key.clone()).expect("Unable to read public key");
    let connection = get_connection(pool).await;
    connection
        .execute(
            "INSERT INTO remote_peer (uid, address, public_key) VALUES (?1, ?2, ?3)",
            (remote.name.clone(), address, public_key),
        )
        .and_then(|nb| {
            debug!("add({:?}) = {}", remote, nb);
            Ok(nb)
        })
        .unwrap_or_else(|e| {
            error!("{e}");
            0
        })
}

pub(crate) async fn remove_by_uid(pool: &Pool, uid: &String) -> usize {
    let connection = get_connection(pool).await;
    connection
        .execute("DELETE FROM remote_peer WHERE uid = ?1", [uid])
        .and_then(|nb| {
            debug!("remove_by_uid({}) = {}", uid, nb);
            Ok(nb)
        })
        .unwrap_or_else(|e| {
            error!("{e}");
            0
        })
}
