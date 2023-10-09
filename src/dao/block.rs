use crate::configuration::DatabaseUpgradeMode;

use super::*;
use std::net::SocketAddr;

#[cfg(feature = "mysql")]
use mysql::{params, Result, Row};
#[cfg(feature = "sqlite")]
use rusqlite::{Result, Row};

enum Queries {
    Drop,
    CreateOrUpgrade,
    SelectAll,
    Add,
    Remove,
}

impl ToSql for Queries {
    fn to_sql(&self) -> &str {
        match self {
            Queries::Drop => "DROP TABLE IF EXISTS block_peer",
            Queries::CreateOrUpgrade => {
                if cfg!(feature = "sqlite") {
                    "CREATE TABLE IF NOT EXISTS block_peer (
                    id INTEGER PRIMARY KEY,
                    address TEXT
                )"
                } else {
                    "CREATE TABLE IF NOT EXISTS block_peer (
                    id INTEGER NOT NULL AUTO_INCREMENT,
                    address TEXT,
                    PRIMARY KEY (id)
                )"
                }
            }
            Queries::SelectAll => "SELECT address FROM block_peer",
            Queries::Add => "INSERT INTO block_peer (address) VALUES (:address)",
            Queries::Remove => "DELETE FROM block_peer WHERE address = :address",
        }
    }
}

#[cfg(feature = "sqlite")]
fn mapper(row: &Row) -> Result<SocketAddr> {
    let address: String = unwrap_result(row.get(0), "Unable to read 'address'");
    Ok(unwrap_result(address.parse(), "Unable to parse 'address'"))
}

#[cfg(feature = "mysql")]
fn mapper(row: &Row) -> Result<SocketAddr> {
    let address: String = unwrap_option(row.get(0), "Unable to read 'address'");
    Ok(unwrap_result(address.parse(), "Unable to parse 'address'"))
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

pub(crate) async fn select_all(pool: &Pool) -> Vec<SocketAddr> {
    prepare(pool, Queries::SelectAll, EMPTY, mapper).await
}

#[cfg(feature = "sqlite")]
pub(crate) async fn add(pool: &Pool, address: &SocketAddr) -> usize {
    execute(
        pool,
        Queries::Add,
        rusqlite::named_params! {
            ":address": address.to_string()
        },
    )
    .await
}

#[cfg(feature = "mysql")]
pub(crate) async fn add(pool: &Pool, address: &SocketAddr) -> usize {
    execute(
        pool,
        Queries::Add,
        mysql::params! {
            "address" => address.to_string()
        },
    )
    .await
}

#[cfg(feature = "sqlite")]
pub(crate) async fn remove(pool: &Pool, address: &SocketAddr) -> usize {
    execute(
        pool,
        Queries::Remove,
        rusqlite::named_params! {
            ":address": address.to_string()
        },
    )
    .await
}

#[cfg(feature = "mysql")]
pub(crate) async fn remove(pool: &Pool, address: &SocketAddr) -> usize {
    execute(
        pool,
        Queries::Remove,
        mysql::params! {
            "address" => address.to_string()
        },
    )
    .await
}
