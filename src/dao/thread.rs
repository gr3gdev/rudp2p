use super::*;

#[cfg(feature = "sqlite")]
pub(crate) async fn create_or_upgrade(pool: &Pool) -> () {
    let sql = "
    CREATE TABLE IF NOT EXISTS thread (
        id INTEGER PRIMARY KEY,
        alive INTEGER
    )";
    execute(pool, sql, {}).await;
    let sql = "INSERT INTO thread (alive) SELECT 0 WHERE NOT EXISTS (SELECT 1 FROM thread)";
    execute(pool, sql, {}).await;
}

#[cfg(feature = "mysql")]
pub(crate) async fn create_or_upgrade(pool: &Pool) {
    let sql = "
    CREATE TABLE IF NOT EXISTS thread (
        id INTEGER NOT NULL AUTO_INCREMENT,
        alive INTEGER,
        PRIMARY KEY (id)
    )";
    execute(pool, sql, {}).await;
    let sql = "INSERT INTO thread (alive) VALUES (0) ON DUPLICATE KEY UPDATE alive=0";
    execute(pool, sql, {}).await;
}

pub(crate) async fn status(pool: &Pool) -> bool {
    let sql = "SELECT alive FROM thread";
    let lines = prepare(pool, sql, {}, |row| {
        let status: bool = row.get(0).expect("Unable to read 'status'");
        Ok(status)
    })
    .await;
    lines.get(0).unwrap().clone()
}

#[cfg(feature = "sqlite")]
pub(crate) async fn update(pool: &Pool, status: bool) -> usize {
    let sql = "UPDATE thread SET alive = ?1";
    execute(pool, sql, [status]).await
}

#[cfg(feature = "mysql")]
pub(crate) async fn update(pool: &Pool, status: bool) -> usize {
    use mysql::params;

    let sql = "UPDATE thread SET alive = :status";
    execute(pool, sql, params! { "status" => status }).await
}
