use super::*;
use crate::peer::RemotePeer;
use std::net::SocketAddr;

#[cfg(feature = "mysql")]
use mysql::{params, Result, Row};
#[cfg(feature = "sqlite")]
use rusqlite::{Result, Row};

enum Queries {
    CreateOrUpgrade,
    SelectAll,
    SelectByAddress,
    Add,
    Remove,
}

impl ToSql for Queries {
    fn to_sql(&self) -> &str {
        match self {
            Queries::CreateOrUpgrade => {
                if cfg!(feature = "mysql") {
                    "CREATE TABLE IF NOT EXISTS remote_peer (
                        id INTEGER NOT NULL AUTO_INCREMENT,
                        address TEXT,
                        public_key TEXT,
                        PRIMARY KEY (id)
                    )"
                } else {
                    "CREATE TABLE IF NOT EXISTS remote_peer (
                        id INTEGER PRIMARY KEY,
                        address TEXT,
                        public_key TEXT
                    )"
                }
            }
            Queries::SelectAll => "SELECT id, address, public_key FROM remote_peer",
            Queries::SelectByAddress => {
                "SELECT id, address, public_key FROM remote_peer WHERE address = :address"
            }
            Queries::Add => {
                "INSERT INTO remote_peer (address, public_key) VALUES (:address, :public_key) RETURNING id, address, public_key"
            }
            Queries::Remove => "DELETE FROM remote_peer WHERE id = :id",
        }
    }
}

fn mapper(row: &Row) -> Result<RemotePeer> {
    let id = row.get(0).expect("Unable to read 'id'");
    let address: String = row.get(1).expect("Unable to read 'address'");
    let public_key: String = row.get(2).expect("Unable to read 'public_key'");
    Ok(RemotePeer {
        id,
        addr: address.parse().expect("Unable to parse address"),
        public_key: public_key.as_bytes().to_vec(),
    })
}

pub(crate) async fn create_or_upgrade(pool: &Pool) {
    execute(pool, Queries::CreateOrUpgrade, EMPTY).await;
}

pub(crate) async fn select_all(pool: &Pool) -> Vec<RemotePeer> {
    prepare(pool, Queries::SelectAll, EMPTY, mapper).await
}

#[cfg(feature = "sqlite")]
pub(crate) async fn select_by_address(pool: &Pool, address: &SocketAddr) -> Vec<RemotePeer> {
    prepare(
        pool,
        Queries::SelectByAddress,
        rusqlite::named_params! {
            ":address": address.to_string()
        },
        mapper,
    )
    .await
}

#[cfg(feature = "mysql")]
pub(crate) async fn select_by_address(pool: &Pool, address: &SocketAddr) -> Vec<RemotePeer> {
    prepare(
        pool,
        Queries::SelectByAddress,
        mysql::params! {
            "address" => address.to_string()
        },
        mapper,
    )
    .await
}

#[cfg(feature = "sqlite")]
pub(crate) async fn add(pool: &Pool, address: &SocketAddr, public_key: &Vec<u8>) -> RemotePeer {
    let pk = String::from_utf8(public_key.clone()).expect("Unable to convert public key to string");
    let res = prepare(
        pool,
        Queries::Add,
        rusqlite::named_params! {
            ":address": address.to_string(),
            ":public_key": pk,
        },
        mapper,
    )
    .await;
    res.get(0).unwrap().clone()
}

#[cfg(feature = "mysql")]
pub(crate) async fn add(pool: &Pool, address: &SocketAddr, public_key: &Vec<u8>) -> RemotePeer {
    let pk = String::from_utf8(public_key.clone()).expect("Unable to convert public key to string");
    let res = prepare(
        pool,
        Queries::Add,
        mysql::params! {
            "address" => address.to_string(),
            "public_key" => pk,
        },
        mapper,
    )
    .await;
    res.get(0).unwrap().clone()
}

#[cfg(feature = "sqlite")]
pub(crate) async fn remove(pool: &Pool, remote: &RemotePeer) -> usize {
    execute(
        pool,
        Queries::Remove,
        rusqlite::named_params! {
            ":id": remote.id
        },
    )
    .await
}

#[cfg(feature = "mysql")]
pub(crate) async fn remove(pool: &Pool, remote: &RemotePeer) -> usize {
    execute(
        pool,
        Queries::Remove,
        mysql::params! {
            "id" => remote.id
        },
    )
    .await
}
