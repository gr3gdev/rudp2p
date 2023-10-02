use super::*;
use std::net::SocketAddr;

#[cfg(feature = "sqlite")]
pub(crate) async fn create_or_upgrade(pool: &Pool) -> () {
    let sql = "
    CREATE TABLE IF NOT EXISTS block_peer (
        id INTEGER PRIMARY KEY,
        address TEXT
    )";
    execute(pool, sql, {}).await;
}

#[cfg(feature = "mysql")]
pub(crate) async fn create_or_upgrade(pool: &Pool) {
    let sql = "
    CREATE TABLE IF NOT EXISTS block_peer (
        id INTEGER NOT NULL AUTO_INCREMENT,
        address TEXT,
        PRIMARY KEY (id)
    )";
    execute(pool, sql, {}).await;
}

pub(crate) async fn select_all(pool: &Pool) -> Vec<SocketAddr> {
    let sql = "SELECT address FROM block_peer";
    prepare(pool, sql, {}, |row| {
        let address: String = row.get(0).expect("Unable to read 'address'");
        Ok(address.parse().expect("Unable to parse address"))
    })
    .await
}

#[cfg(feature = "sqlite")]
pub(crate) async fn add(pool: &Pool, address: &SocketAddr) -> usize {
    let sql = "INSERT INTO block_peer (address) VALUES (?1)";
    execute(pool, sql, [address.to_string()]).await
}

#[cfg(feature = "mysql")]
pub(crate) async fn add(pool: &Pool, address: &SocketAddr) -> usize {
    use mysql::params;

    let sql = "INSERT INTO block_peer (address) VALUES (:address)";
    execute(pool, sql, params! { "address" => address.to_string() }).await
}

#[cfg(feature = "sqlite")]
pub(crate) async fn remove(pool: &Pool, address: &SocketAddr) -> usize {
    let sql = "DELETE FROM block_peer WHERE address = ?1";
    execute(pool, sql, [address.to_string()]).await
}

#[cfg(feature = "mysql")]
pub(crate) async fn remove(pool: &Pool, address: &SocketAddr) -> usize {
    use mysql::params;

    let sql = "DELETE FROM block_peer WHERE address = :address";
    execute(pool, sql, params! { "address" => address.to_string() }).await
}
