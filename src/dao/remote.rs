use super::*;
use crate::peer::RemotePeer;
use std::net::SocketAddr;

#[cfg(feature = "sqlite")]
pub(crate) async fn create_or_upgrade(pool: &Pool) {
    let sql = "
    CREATE TABLE IF NOT EXISTS remote_peer (
        id INTEGER PRIMARY KEY,
        address TEXT,
        public_key TEXT
    )";
    execute(pool, sql, {}).await;
}

#[cfg(feature = "mysql")]
pub(crate) async fn create_or_upgrade(pool: &Pool) {
    let sql = "
    CREATE TABLE IF NOT EXISTS remote_peer (
        id INTEGER NOT NULL AUTO_INCREMENT,
        address TEXT,
        public_key TEXT,
        PRIMARY KEY (id)
    )";
    execute(pool, sql, {}).await;
}

#[cfg(feature = "sqlite")]
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

#[cfg(feature = "mysql")]
fn mapper(row: mysql::Row) -> RemotePeer {
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
    let sql = "SELECT id, address, public_key FROM remote_peer";
    prepare(pool, sql, {}, |row| Ok(mapper(row))).await
}

#[cfg(feature = "sqlite")]
pub(crate) async fn select_by_address(pool: &Pool, address: &SocketAddr) -> Vec<RemotePeer> {
    let sql = "SELECT id, address, public_key FROM remote_peer WHERE address = ?1";
    prepare(pool, sql, [address.to_string()], |row| Ok(mapper(row))).await
}

#[cfg(feature = "mysql")]
pub(crate) async fn select_by_address(pool: &Pool, address: &SocketAddr) -> Vec<RemotePeer> {
    use mysql::params;

    let sql = "SELECT id, address, public_key FROM remote_peer WHERE address = :address";
    prepare(
        pool,
        sql,
        params! { "address" => address.to_string() },
        |row| Ok(mapper(row)),
    )
    .await
}

pub(crate) async fn add(pool: &Pool, address: &SocketAddr, public_key: &Vec<u8>) -> RemotePeer {
    let sql = "INSERT INTO remote_peer (address, public_key) VALUES (?1, ?2) RETURNING id";
    let pk = String::from_utf8(public_key.clone()).expect("Unable to convert public key to string");
    let res = prepare(pool, sql, (address.to_string(), pk), |row| {
        let id = row.get(0).expect("Unable to read 'id'");
        Ok(RemotePeer {
            id,
            addr: address.clone(),
            public_key: public_key.clone(),
        })
    })
    .await;
    res.get(0).unwrap().clone()
}

#[cfg(feature = "sqlite")]
pub(crate) async fn remove(pool: &Pool, remote: &RemotePeer) -> () {
    let sql = "DELETE FROM remote_peer WHERE id = ?1";
    execute(pool, sql, [remote.id]).await
}

#[cfg(feature = "mysql")]
pub(crate) async fn remove(pool: &Pool, remote: &RemotePeer) -> () {
    use mysql::params;

    let sql = "DELETE FROM remote_peer WHERE id = :id";
    execute(pool, sql, params! { "id" => remote.id }).await
}
