use super::*;

#[cfg(feature = "mysql")]
use mysql::{params, Result, Row};
#[cfg(feature = "sqlite")]
use rusqlite::{Result, Row};

enum Queries {
    CreateOrUpgrade,
    Add,
    Status,
    Update,
    Delete,
}

impl ToSql for Queries {
    fn to_sql(&self) -> &str {
        match self {
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

fn mapper(row: &Row) -> Result<bool> {
    let status: bool = row.get(0).expect("Unable to read 'status'");
    Ok(status)
}

pub(crate) async fn create_or_upgrade(pool: &Pool) {
    execute(pool, Queries::CreateOrUpgrade, EMPTY).await;
    execute(pool, Queries::Delete, EMPTY).await;
    execute(pool, Queries::Add, EMPTY).await;
}

pub(crate) async fn status(pool: &Pool) -> bool {
    let lines = prepare(pool, Queries::Status, EMPTY, mapper).await;
    lines.get(0).unwrap().clone()
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
