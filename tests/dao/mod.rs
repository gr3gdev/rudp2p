use async_trait::async_trait;
use log::debug;
use rudp2plib::{
    dao::PeerDao,
    network::request::{RequestPart, Type},
    peer::RemotePeer,
};
use rusqlite::{Params, Row};
use std::net::SocketAddr;

pub(crate) type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;
pub(crate) type Connection = r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>;

#[derive(Debug, Clone)]
pub(crate) struct SqlitePeerDao {
    pool: Pool,
}

impl SqlitePeerDao {
    pub(crate) fn new(pool: &Pool) -> Self {
        Self { pool: pool.clone() }
    }
}

enum Queries {
    CreateTableRemotePeer,
    CreateTableRequestPart,
    CreateTableThread,
    CreateTableBlock,
    InsertRemotePeer,
    InsertRequestPart,
    InsertBlock,
    InsertThread,
    UpdateThread,
    DeleteRemotePeer,
    DeleteRequestPart,
    DeleteBlock,
    SelectRemotePeerByAddress,
    SelectAllRemotePeer,
    SelectRequestPartByUid,
    SelectBlock,
    SelectThread,
}

impl Queries {
    fn sql(&self) -> &str {
        match self {
            Queries::CreateTableRemotePeer => {
                if cfg!(feature = "ssl") {
                    "CREATE TABLE remote_peer (address TEXT PRIMARY KEY, public_key TEXT)"
                } else {
                    "CREATE TABLE remote_peer (address TEXT PRIMARY KEY)"
                }
            },
            Queries::CreateTableRequestPart => "CREATE TABLE request_part (id INTEGER PRIMARY KEY, uid TEXT, request_type INTEGER, start INTEGER, total INTEGER, content_size INTEGER, content TEXT, sender TEXT)",
            Queries::CreateTableThread => "CREATE TABLE thread (id INTEGER PRIMARY KEY, status INTEGER)",
            Queries::CreateTableBlock => "CREATE TABLE block (address TEXT PRIMARY KEY)",
            Queries::InsertRemotePeer => {
                if cfg!(feature = "ssl") {
                    "INSERT OR IGNORE INTO remote_peer (address, public_key) VALUES (?1, ?2)"
                } else {
                    "INSERT OR IGNORE INTO remote_peer (address) VALUES (?1)"
                }
            },
            Queries::InsertRequestPart => "INSERT INTO request_part (uid, request_type, start, total, content_size, content, sender) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            Queries::InsertBlock => "INSERT OR IGNORE INTO block (address) VALUES (?1)",
            Queries::InsertThread => "INSERT OR IGNORE INTO thread (status) VALUES (false)",
            Queries::UpdateThread => "UPDATE thread SET status = ?1",
            Queries::DeleteRemotePeer => "DELETE FROM remote_peer WHERE address = ?1",
            Queries::DeleteRequestPart => "DELETE FROM request_part WHERE uid = ?1",
            Queries::DeleteBlock => "DELETE FROM block WHERE address = ?1",
            Queries::SelectRemotePeerByAddress => "SELECT * FROM remote_peer WHERE address = ?1",
            Queries::SelectAllRemotePeer => "SELECT * FROM remote_peer",
            Queries::SelectRequestPartByUid => "SELECT * FROM request_part WHERE uid = ?1",
            Queries::SelectBlock => "SELECT * FROM block",
            Queries::SelectThread => "SELECT * FROM thread",
        }
    }
}

async fn execute<P: Params>(pool: &Pool, query: Queries, params: P) -> usize {
    let connection = get_connection(pool).await;
    connection
        .execute(query.sql(), params)
        .or_else(|e| {
            log::error!("{e}");
            Err(e)
        })
        .unwrap()
}

fn execute_with_connection<P: Params>(
    connection: &Connection,
    query: Queries,
    params: P,
) -> usize {
    connection
        .execute(query.sql(), params)
        .or_else(|e| {
            log::error!("{e}");
            Err(e)
        })
        .unwrap()
}

async fn prepare<P: Params, T>(
    pool: &Pool,
    query: Queries,
    params: P,
    mapper: impl Fn(&Row) -> T,
) -> Vec<T> {
    let connection = get_connection(pool).await;
    let mut statement = connection
        .prepare(query.sql())
        .or_else(|e| {
            log::error!("{e}");
            Err(e)
        })
        .unwrap();
    statement
        .query_map(params, |row| Ok(mapper(row)))
        .and_then(Iterator::collect)
        .or_else(|e| {
            log::error!("{e}");
            Err(e)
        })
        .unwrap()
}

#[async_trait]
impl PeerDao for SqlitePeerDao {
    async fn init(&mut self) -> () {
        let connection = get_connection(&self.pool).await;
        execute_with_connection(&connection, Queries::CreateTableRemotePeer, []);
        execute_with_connection(&connection, Queries::CreateTableRequestPart, []);
        execute_with_connection(&connection, Queries::CreateTableBlock, []);
        execute_with_connection(&connection, Queries::CreateTableThread, []);
        execute_with_connection(&connection, Queries::InsertThread, []);
    }

    #[cfg(feature = "ssl")]
    async fn add_remote(&self, address: &SocketAddr, public_key: &Vec<u8>) -> RemotePeer {
        execute(
            &self.pool,
            Queries::InsertRemotePeer,
            (address.to_string(), public_key),
        )
        .await;
        RemotePeer::new(address, public_key)
    }
    #[cfg(not(feature = "ssl"))]
    async fn add_remote(&self, address: &SocketAddr) -> RemotePeer {
        execute(&self.pool, Queries::InsertRemotePeer, [address.to_string()]).await;
        RemotePeer::new(address)
    }

    async fn remove_remote(&self, remote: &RemotePeer) -> usize {
        execute(
            &self.pool,
            Queries::DeleteRemotePeer,
            [remote.addr.to_string()],
        )
        .await
    }

    async fn find_remotes_by_address(&self, address: &SocketAddr) -> Vec<RemotePeer> {
        prepare(
            &self.pool,
            Queries::SelectRemotePeerByAddress,
            [address.to_string()],
            remote_peer_mapper,
        )
        .await
    }

    async fn find_all_remotes(&self) -> Vec<RemotePeer> {
        prepare(
            &self.pool,
            Queries::SelectAllRemotePeer,
            [],
            remote_peer_mapper,
        )
        .await
    }

    async fn add_request_part(&self, part: &RequestPart) -> usize {
        execute(
            &self.pool,
            Queries::InsertRequestPart,
            (
                part.uid.clone(),
                part.request_type.to_code(),
                part.start,
                part.total,
                part.content_size,
                part.content.clone(),
                part.sender.to_string(),
            ),
        )
        .await
    }

    async fn remove_request_part_by_uid(&self, uid: &String) -> usize {
        execute(&self.pool, Queries::DeleteRequestPart, [uid]).await
    }

    async fn find_requests_part_by_uid(&self, uid: &String) -> Vec<RequestPart> {
        prepare(
            &self.pool,
            Queries::SelectRequestPartByUid,
            [uid],
            request_part_mapper,
        )
        .await
    }

    async fn block(&self, address: &SocketAddr) -> usize {
        execute(&self.pool, Queries::InsertBlock, [address.to_string()]).await
    }

    async fn unblock(&self, address: &SocketAddr) -> usize {
        execute(&self.pool, Queries::DeleteBlock, [address.to_string()]).await
    }

    async fn find_all_block(&self) -> Vec<SocketAddr> {
        prepare(&self.pool, Queries::SelectBlock, [], block_mapper).await
    }

    async fn update_status(&self, value: bool) -> () {
        execute(&self.pool, Queries::UpdateThread, [value]).await;
    }

    async fn find_status(&self) -> bool {
        let lines = prepare(&self.pool, Queries::SelectThread, [], thread_mapper).await;
        lines.first().unwrap().clone()
    }
}

fn thread_mapper(row: &rusqlite::Row) -> bool {
    row.get(0).unwrap()
}

fn block_mapper(row: &rusqlite::Row) -> SocketAddr {
    let addr_string: String = row.get(0).unwrap();
    addr_string.parse().unwrap()
}

fn request_part_mapper(row: &rusqlite::Row) -> RequestPart {
    let addr_string: String = row.get(7).unwrap();
    RequestPart {
        uid: row.get(1).unwrap(),
        request_type: Type::from_code(row.get(2).unwrap()),
        start: row.get(3).unwrap(),
        total: row.get(4).unwrap(),
        content_size: row.get(5).unwrap(),
        content: row.get(6).unwrap(),
        sender: addr_string.parse().unwrap(),
    }
}

#[cfg(not(feature = "ssl"))]
fn remote_peer_mapper(row: &rusqlite::Row) -> RemotePeer {
    let addr_string: String = row.get(0).unwrap();
    RemotePeer::new(&addr_string.parse().unwrap())
}

#[cfg(feature = "ssl")]
fn remote_peer_mapper(row: &rusqlite::Row) -> RemotePeer {
    let addr_string: String = row.get(0).unwrap();
    RemotePeer::new(&addr_string.parse().unwrap(), &row.get(1).unwrap())
}

#[derive(Debug)]
pub(crate) struct ConnectedEvent {
    pub(crate) from: SocketAddr,
    pub(crate) to: String,
}

#[derive(Debug)]
pub(crate) struct DisconnectedEvent {
    pub(crate) from: SocketAddr,
    pub(crate) to: String,
}

#[derive(Debug)]
pub(crate) struct MessageEvent {
    pub(crate) from: SocketAddr,
    pub(crate) to: String,
    pub(crate) content: Vec<u8>,
}

async fn get_connection(pool: &Pool) -> Connection {
    let pool = pool.clone();
    pool.get()
        .or_else(|e| {
            log::error!("{e}");
            Err(e)
        })
        .unwrap()
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
            (c.from.to_string(), c.to),
        )
        .unwrap()
}

pub(crate) async fn is_peer_connected_with(pool: &Pool, peer: &String, other: &SocketAddr) -> bool {
    let connection = get_connection(pool).await;
    let mut statement = connection
        .prepare("SELECT 1 FROM connections WHERE to_peer = ?1 AND from_peer = ?2")
        .expect("Unable to prepare query : get_connection_for_peer");
    let mut rows = statement.query((peer, other.to_string())).unwrap();
    rows.next().unwrap().is_some()
}

pub(crate) async fn add_disconnection(pool: &Pool, d: DisconnectedEvent) -> usize {
    let connection = get_connection(pool).await;
    connection
        .execute(
            "INSERT INTO disconnections (from_peer, to_peer) VALUES (?1, ?2)",
            (d.from.to_string(), d.to),
        )
        .unwrap()
}

pub(crate) async fn is_peer_disconnected_with(
    pool: &Pool,
    peer: &String,
    other: &SocketAddr,
) -> bool {
    let connection = get_connection(pool).await;
    let mut statement = connection
        .prepare("SELECT 1 FROM disconnections WHERE to_peer = ?1 AND from_peer = ?2")
        .expect("Unable to prepare query : get_connection_for_peer");
    let mut rows = statement.query((peer, other.to_string())).unwrap();
    rows.next().unwrap().is_some()
}

pub(crate) async fn add_message(pool: &Pool, m: MessageEvent) -> usize {
    let connection = get_connection(pool).await;
    connection
        .execute(
            "INSERT INTO messages (from_peer, to_peer, content) VALUES (?1, ?2, ?3)",
            (m.from.to_string(), m.to, m.content),
        )
        .unwrap()
}

pub(crate) async fn get_peer_messages_from(
    pool: &Pool,
    peer: &String,
    from: &SocketAddr,
) -> Vec<MessageEvent> {
    let connection = get_connection(pool).await;
    let mut statement = connection
        .prepare("SELECT from_peer, to_peer, content FROM messages WHERE to_peer = ?1 AND from_peer = ?2")
        .expect("Unable to prepare query : get_message_for_peer");
    statement
        .query_map((peer, from.to_string()), |row| {
            let addr: String = row.get(0).unwrap();
            let message = MessageEvent {
                from: addr.parse().unwrap(),
                to: row.get(1).unwrap(),
                content: row.get(2).unwrap(),
            };
            debug!(
                "[PEER {peer}] [TEST] get_peer_messages_from({from}) : {:?}",
                message
            );
            Ok(message)
        })
        .and_then(Iterator::collect)
        .unwrap()
}
