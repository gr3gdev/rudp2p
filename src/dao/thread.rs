use super::*;
use log::{error, trace};

pub(crate) async fn create_or_upgrade(connection: &Connection) {
    connection
        .execute(
            "
    CREATE TABLE IF NOT EXISTS thread (
        id INTEGER PRIMARY KEY,
        alive INTEGER
    )",
            [],
        )
        .expect("Unable to create table 'thread'");
    connection
        .execute(
            "INSERT INTO thread (alive) SELECT 0 WHERE NOT EXISTS (SELECT 1 FROM thread)",
            [],
        )
        .unwrap();
}

pub(crate) async fn status(pool: &Pool) -> bool {
    let connection = get_connection(pool).await;
    let mut statement = connection
        .prepare("SELECT alive FROM thread")
        .expect("Unable to prepare query : status");
    let lines = statement
        .query_map([], |row| {
            let status: bool = row.get(0).expect("Unable to read 'status'");
            trace!("[DAO] thread::status() = {:?}", status);
            Ok(status)
        })
        .and_then(Iterator::collect)
        .unwrap_or_else(|e| {
            error!("status - {:?} - {e}", pool);
            vec![]
        });
    lines.get(0).unwrap().clone()
}

pub(crate) async fn update(pool: &Pool, status: bool) -> usize {
    let connection = get_connection(pool).await;
    connection
        .execute("UPDATE thread SET alive = ?1", [status])
        .and_then(|nb| {
            trace!(
                "[DAO] thread::update({:?}) = {nb}",
                status
            );
            Ok(nb)
        })
        .unwrap_or_else(|e| {
            error!("update - {:?} - {e}", pool);
            0
        })
}
