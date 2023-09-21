use super::*;
use log::{error, trace};
use std::net::SocketAddr;

pub(crate) async fn create_or_upgrade(connection: &Connection) {
    connection
        .execute(
            "
    CREATE TABLE IF NOT EXISTS block_peer (
        id INTEGER PRIMARY KEY,
        address TEXT
    )",
            [],
        )
        .expect("Unable to create table 'block_peer'");
}

pub(crate) async fn select_all(peer_uid: &String, pool: &Pool) -> Vec<SocketAddr> {
    let connection = get_connection(pool).await;
    let mut statement = connection
        .prepare("SELECT address FROM block_peer")
        .expect("Unable to prepare query : select_all");
    statement
        .query_map([], |row| {
            let address: String = row.get(0).expect("Unable to read 'address'");
            trace!(
                "[PEER {peer_uid}] [DAO] block::select_all() = {:?}",
                address
            );
            Ok(address.parse().expect("Unable to parse address"))
        })
        .and_then(Iterator::collect)
        .unwrap_or_else(|e| {
            error!("{e}");
            vec![]
        })
}

pub(crate) async fn add(peer_uid: &String, pool: &Pool, address: &SocketAddr) -> usize {
    let address = &address.to_string();
    let connection = get_connection(pool).await;
    connection
        .execute("INSERT INTO block_peer (address) VALUES (?1)", [address])
        .and_then(|nb| {
            trace!("[PEER {peer_uid}] [DAO] block::add({:?}) = {nb}", address);
            Ok(nb)
        })
        .unwrap_or_else(|e| {
            error!("{e}");
            0
        })
}
