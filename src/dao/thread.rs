use crate::configuration::DatabaseUpgradeMode;

use super::*;

#[cfg(feature = "mysql")]
use mysql::{params, Result, Row};
#[cfg(feature = "sqlite")]
use rusqlite::{Result, Row};

enum Queries {
    Drop,
    CreateOrUpgrade,
    Add,
    Status,
    Update,
    Delete,
}

impl ToSql for Queries {
    fn to_sql(&self) -> &str {
        match self {
            Queries::Drop => "DROP TABLE IF EXISTS thread",
            Queries::CreateOrUpgrade => {
                if cfg!(feature = "mysql") {
                    "CREATE TABLE IF NOT EXISTS thread (
                        id INTEGER NOT NULL AUTO_INCREMENT,
                        alive INTEGER,
                        PRIMARY KEY (id)
                    )"
                } else {
                    "CREATE TABLE IF NOT EXISTS thread (
                        id INTEGER PRIMARY KEY,
                        alive INTEGER
                    )"
                }
            }
            Queries::Add => "INSERT INTO thread (alive) VALUES (0)",
            Queries::Status => "SELECT alive FROM thread",
            Queries::Update => "UPDATE thread SET alive = :status",
            Queries::Delete => "DELETE FROM thread",
        }
    }
}

#[cfg(feature = "sqlite")]
fn mapper(row: &Row) -> Result<bool> {
    let status: bool = unwrap_result(row.get(0), "Unable to read 'status'");
    Ok(status)
}

#[cfg(feature = "mysql")]
fn mapper(row: &Row) -> Result<bool> {
    let status: bool = unwrap_option(row.get(0), "Unable to read 'status'");
    Ok(status)
}

pub(crate) async fn create_or_upgrade(pool: &Pool, database_upgrade_mode: &DatabaseUpgradeMode) {
    match database_upgrade_mode {
        DatabaseUpgradeMode::AlwaysNew => {
            execute(pool, Queries::Drop, EMPTY).await;
            execute(pool, Queries::CreateOrUpgrade, EMPTY).await;
            execute(pool, Queries::Add, EMPTY).await;
        }
        DatabaseUpgradeMode::Upgrade => {
            execute(pool, Queries::CreateOrUpgrade, EMPTY).await;
            execute(pool, Queries::Delete, EMPTY).await;
            execute(pool, Queries::Add, EMPTY).await;
        }
    }
}

pub(crate) async fn status(pool: &Pool) -> bool {
    let lines = prepare(pool, Queries::Status, EMPTY, mapper).await;
    unwrap_option(
        lines.first().map(bool::clone),
        "Unable to get the first line of thread",
    )
}

#[cfg(feature = "sqlite")]
pub(crate) async fn update(pool: &Pool, status: bool) -> usize {
    execute(
        pool,
        Queries::Update,
        rusqlite::named_params! {
            ":status": status,
        },
    )
    .await
}

#[cfg(feature = "mysql")]
pub(crate) async fn update(pool: &Pool, status: bool) -> usize {
    execute(
        pool,
        Queries::Update,
        mysql::params! {
            "status" => status,
        },
    )
    .await
}
