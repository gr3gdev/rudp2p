use serde::{Deserialize, Serialize};

pub(crate) type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;
pub(crate) type Connection = r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>;

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct ConnectedEvent {
    pub(crate) from: String,
    pub(crate) to: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct DisconnectedEvent {
    pub(crate) from: String,
    pub(crate) to: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct MessageEvent {
    pub(crate) from: String,
    pub(crate) to: String,
    pub(crate) content: Vec<u8>,
}

async fn get_connection(pool: &Pool) -> Connection {
    let pool = pool.clone();
    pool.get().expect("Unable to obtain pool")
}

async fn create_connections_table(connection: &Connection) {
    connection
        .execute(
            "
    CREATE TABLE IF NOT EXISTS connections (
        id INTEGER PRIMARY KEY,
        from_peer TEXT,
        to_peer TEXT
    )",
            [],
        )
        .expect("Unable to create table 'connections'");
}

async fn create_disconnections_table(connection: &Connection) {
    connection
        .execute(
            "
    CREATE TABLE IF NOT EXISTS disconnections (
        id INTEGER PRIMARY KEY,
        from_peer TEXT,
        to_peer TEXT
    )",
            [],
        )
        .expect("Unable to create table 'disconnections'");
}

async fn create_messages_table(connection: &Connection) {
    connection
        .execute(
            "
    CREATE TABLE IF NOT EXISTS messages (
        id INTEGER PRIMARY KEY,
        from_peer TEXT,
        to_peer TEXT,
        content TEXT
    )",
            [],
        )
        .expect("Unable to create table 'messages'");
}

pub(crate) async fn init(pool: &Pool) {
    let connection = get_connection(pool).await;
    create_connections_table(&connection).await;
    create_disconnections_table(&connection).await;
    create_messages_table(&connection).await;
}

pub(crate) async fn add_connection(pool: &Pool, c: ConnectedEvent) -> usize {
    let connection = get_connection(pool).await;
    connection
        .execute(
            "INSERT INTO connections (from_peer, to_peer) VALUES (?1, ?2)",
            (c.from, c.to),
        )
        .unwrap()
}

pub(crate) async fn is_peer_connected_with(pool: &Pool, peer: &String, other: &String) -> bool {
    let connection = get_connection(pool).await;
    let mut statement = connection
        .prepare("SELECT 1 FROM connections WHERE to_peer = ?1 AND from_peer = ?2")
        .expect("Unable to prepare query : get_connection_for_peer");
    let mut rows = statement.query((peer, other)).unwrap();
    rows.next().unwrap().is_some()
}

pub(crate) async fn add_disconnection(pool: &Pool, d: DisconnectedEvent) -> usize {
    let connection = get_connection(pool).await;
    connection
        .execute(
            "INSERT INTO disconnections (from_peer, to_peer) VALUES (?1, ?2)",
            (d.from, d.to),
        )
        .unwrap()
}

pub(crate) async fn is_peer_disconnected_with(pool: &Pool, peer: &String, other: &String) -> bool {
    let connection = get_connection(pool).await;
    let mut statement = connection
        .prepare("SELECT 1 FROM disconnections WHERE to_peer = ?1 AND from_peer = ?2")
        .expect("Unable to prepare query : get_connection_for_peer");
    let mut rows = statement.query((peer, other)).unwrap();
    rows.next().unwrap().is_some()
}

pub(crate) async fn add_message(pool: &Pool, m: MessageEvent) -> usize {
    let connection = get_connection(pool).await;
    connection
        .execute(
            "INSERT INTO messages (from_peer, to_peer, content) VALUES (?1, ?2, ?3)",
            (m.from, m.to, m.content),
        )
        .unwrap()
}

pub(crate) async fn get_peer_messages_from(pool: &Pool, peer: &String, from: &String) -> Vec<MessageEvent> {
    let connection = get_connection(pool).await;
    let mut statement = connection
        .prepare("SELECT from_peer, to_peer, content FROM messages WHERE to_peer = ?1 AND from_peer = ?2")
        .expect("Unable to prepare query : get_message_for_peer");
    statement
        .query_map((peer, from), |row| {
            Ok(MessageEvent {
                from: row.get(0).unwrap(),
                to: row.get(1).unwrap(),
                content: row.get(2).unwrap(),
            })
        })
        .and_then(Iterator::collect)
        .unwrap()
}
