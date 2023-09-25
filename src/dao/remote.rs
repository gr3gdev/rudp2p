use crate::peer::RemotePeer;

use super::*;
use log::{error, trace};
use std::net::SocketAddr;

pub(crate) async fn create_or_upgrade(connection: &Connection) {
    connection
        .execute(
            "
    CREATE TABLE IF NOT EXISTS remote_peer (
        id INTEGER PRIMARY KEY,
        address TEXT,
        public_key TEXT
    )",
            [],
        )
        .expect("Unable to create table 'remote_peer'");
}

fn mapper(row: &rusqlite::Row<'_>) -> RemotePeer {
    let id = row.get(0).expect("Unable to read 'id'");
    let address: String = row.get(1).expect("Unable to read 'address'");
    let public_key: String = row.get(2).expect("Unable to read 'public_key'");
    RemotePeer {
        id,
        addr: address.parse().expect("Unable to parse address"),
        public_key: public_key.as_bytes().to_vec(),
    }
}

pub(crate) async fn select_all(pool: &Pool) -> Vec<RemotePeer> {
    let connection = get_connection(pool).await;
    let mut statement = connection
        .prepare("SELECT id, address, public_key FROM remote_peer")
        .expect("Unable to prepare query : select_all");
    statement
        .query_map([], |row| {
            let remote = mapper(row);
            trace!("[DAO] remote::select_all() = {:?}", remote);
            Ok(remote)
        })
        .and_then(Iterator::collect)
        .unwrap_or_else(|e| {
            error!("select_all - {:?} - {e}", pool);
            vec![]
        })
}

pub(crate) async fn select_by_address(pool: &Pool, address: &SocketAddr) -> Vec<RemotePeer> {
    let connection = get_connection(pool).await;
    let mut statement = connection
        .prepare("SELECT id, address, public_key FROM remote_peer WHERE address = ?1")
        .expect("Unable to prepare query : select_by_remote");
    statement
        .query_map([address.to_string()], |row| {
            let res = mapper(row);
            trace!("[DAO] remote::select_by_address({:?}) = {:?}", address, res);
            Ok(res)
        })
        .and_then(Iterator::collect)
        .unwrap_or_else(|e| {
            error!("select_by_address - {:?} - {e}", pool);
            vec![]
        })
}

pub(crate) async fn add(pool: &Pool, address: &SocketAddr, public_key: &Vec<u8>) -> RemotePeer {
    let pk = String::from_utf8(public_key.clone()).expect("Unable to convert public key to string");
    let connection = get_connection(pool).await;
    let mut statement = connection
        .prepare("INSERT INTO remote_peer (address, public_key) VALUES (?1, ?2) RETURNING id")
        .expect("Unable to prepare query : add");
    let res = statement
        .query_map((address.to_string(), pk), |row| {
            let id = row.get(0).expect("Unable to read 'id'");
            let remote = RemotePeer {
                id,
                addr: address.clone(),
                public_key: public_key.clone(),
            };
            trace!("[DAO] remote::add({address}) = {:?}", remote);
            Ok(remote)
        })
        .and_then(Iterator::collect)
        .unwrap_or_else(|e| {
            error!("add - {:?} - {e}", pool);
            vec![]
        });
    res.get(0).unwrap().clone()
}

pub(crate) async fn remove(pool: &Pool, remote: &RemotePeer) -> usize {
    let connection = get_connection(pool).await;
    connection
        .execute("DELETE FROM remote_peer WHERE id = ?1", [remote.id])
        .and_then(|nb| {
            trace!("[DAO] remote::remove({:?}) = {nb}", remote);
            Ok(nb)
        })
        .unwrap_or_else(|e| {
            error!("remove - {:?} - {e}", pool);
            0
        })
}
