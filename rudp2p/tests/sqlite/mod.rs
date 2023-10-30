use async_trait::async_trait;
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

pub(crate) async fn get_connection(pool: &Pool) -> Connection {
    let pool = pool.clone();
    pool.get()
        .or_else(|e| {
            log::error!("{e}");
            Err(e)
        })
        .unwrap()
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
        execute(&self.pool, Queries::CreateTableRemotePeer, []).await;
        execute(&self.pool, Queries::CreateTableRequestPart, []).await;
        execute(&self.pool, Queries::CreateTableBlock, []).await;
        execute(&self.pool, Queries::CreateTableThread, []).await;
        execute(&self.pool, Queries::InsertThread, []).await;
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

fn thread_mapper(row: &Row) -> bool {
    row.get(0).unwrap()
}

fn block_mapper(row: &Row) -> SocketAddr {
    let addr_string: String = row.get(0).unwrap();
    addr_string.parse().unwrap()
}

fn request_part_mapper(row: &Row) -> RequestPart {
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
fn remote_peer_mapper(row: &Row) -> RemotePeer {
    let addr_string: String = row.get(0).unwrap();
    RemotePeer::new(&addr_string.parse().unwrap())
}

#[cfg(feature = "ssl")]
fn remote_peer_mapper(row: &Row) -> RemotePeer {
    let addr_string: String = row.get(0).unwrap();
    RemotePeer::new(&addr_string.parse().unwrap(), &row.get(1).unwrap())
}
