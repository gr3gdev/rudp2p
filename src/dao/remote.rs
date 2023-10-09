use super::*;
use crate::{peer::RemotePeer, configuration::DatabaseUpgradeMode};
use std::net::SocketAddr;

#[cfg(feature = "mysql")]
use mysql::{params, Result, Row};
#[cfg(feature = "sqlite")]
use rusqlite::{Result, Row};

enum Queries {
    Drop,
    CreateOrUpgrade,
    SelectAll,
    SelectByAddress,
    Add,
    Remove,
}

impl ToSql for Queries {
    fn to_sql(&self) -> &str {
        match self {
            Queries::Drop => "DROP TABLE IF EXISTS remote_peer",
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

#[cfg(all(not(feature = "ssl"), feature = "sqlite"))]
fn mapper(row: &Row) -> Result<RemotePeer> {
    let id = unwrap_result(row.get(0), "Unable to read 'id'");
    let address: String = unwrap_result(row.get(1), "Unable to read 'address'");
    Ok(RemotePeer {
        id,
        addr: unwrap_result(address.parse(), "Unable to parse address"),
    })
}

#[cfg(all(not(feature = "ssl"), feature = "mysql"))]
fn mapper(row: &Row) -> Result<RemotePeer> {
    let id = unwrap_option(row.get(0), "Unable to read 'id'");
    let address: String = unwrap_option(row.get(1), "Unable to read 'address'");
    Ok(RemotePeer {
        id,
        addr: unwrap_result(address.parse(), "Unable to parse address"),
    })
}

#[cfg(all(feature = "ssl", feature = "sqlite"))]
fn mapper(row: &Row) -> Result<RemotePeer> {
    let id = unwrap_result(row.get(0), "Unable to read 'id'");
    let address: String = unwrap_result(row.get(1), "Unable to read 'address'");
    let public_key: String = unwrap_result(row.get(2), "Unable to read 'public_key'");
    Ok(RemotePeer {
        id,
        addr: unwrap_result(address.parse(), "Unable to parse address"),
        public_key: public_key.as_bytes().to_vec(),
    })
}

#[cfg(all(feature = "ssl", feature = "mysql"))]
fn mapper(row: &Row) -> Result<RemotePeer> {
    let id = unwrap_option(row.get(0), "Unable to read 'id'");
    let address: String = unwrap_option(row.get(1), "Unable to read 'address'");
    let public_key: String = unwrap_option(row.get(2), "Unable to read 'public_key'");
    Ok(RemotePeer {
        id,
        addr: unwrap_result(address.parse(), "Unable to parse address"),
        public_key: public_key.as_bytes().to_vec(),
    })
}

pub(crate) async fn create_or_upgrade(pool: &Pool, database_upgrade_mode: &DatabaseUpgradeMode) {
    match database_upgrade_mode {
        DatabaseUpgradeMode::Upgrade => execute(pool, Queries::CreateOrUpgrade, EMPTY).await,
        DatabaseUpgradeMode::AlwaysNew => {
            execute(pool, Queries::Drop, EMPTY).await;
            execute(pool, Queries::CreateOrUpgrade, EMPTY).await
        }
    };
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
    let pk = unwrap_result(
        String::from_utf8(public_key.clone()),
        "Unable to convert public key to string",
    );
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
    unwrap_option(
        res.first().map(RemotePeer::clone),
        "Unable to get the remote peer added",
    )
}

#[cfg(feature = "mysql")]
pub(crate) async fn add(pool: &Pool, address: &SocketAddr, public_key: &Vec<u8>) -> RemotePeer {
    let pk = unwrap_result(
        String::from_utf8(public_key.clone()),
        "Unable to convert public key to string",
    );
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
    unwrap_option(
        res.first().map(RemotePeer::clone),
        "Unable to get the remote peer added",
    )
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
