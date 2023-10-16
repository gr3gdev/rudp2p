use async_trait::async_trait;
use mysql::params;
use mysql::{prelude::Queryable, Params, Row};
use rudp2plib::{
    dao::PeerDao,
    network::request::{RequestPart, Type},
    peer::RemotePeer,
};
use std::net::SocketAddr;

pub(crate) type Pool = r2d2::Pool<r2d2_mysql::MySqlConnectionManager>;
pub(crate) type Connection = r2d2::PooledConnection<r2d2_mysql::MySqlConnectionManager>;

#[derive(Debug, Clone)]
pub(crate) struct MysqlPeerDao {
    pool: Pool,
}

impl MysqlPeerDao {
    pub(crate) fn new(pool: &Pool) -> Self {
        Self { pool: pool.clone() }
    }
}

enum Queries {
    DropTableRemotePeer,
    DropTableRequestPart,
    DropTableThread,
    DropTableBlock,
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
                    "CREATE TABLE remote_peer (address VARCHAR(24) PRIMARY KEY, public_key BLOB)"
                } else {
                    "CREATE TABLE remote_peer (address VARCHAR(24) PRIMARY KEY)"
                }
            },
            Queries::CreateTableRequestPart => "CREATE TABLE request_part (id INTEGER AUTO_INCREMENT PRIMARY KEY, uid VARCHAR(64), request_type INTEGER, start INTEGER, total INTEGER, content_size INTEGER, content BLOB, sender VARCHAR(64))",
            Queries::CreateTableThread => "CREATE TABLE thread (id INTEGER PRIMARY KEY, status INTEGER)",
            Queries::CreateTableBlock => "CREATE TABLE block (address VARCHAR(24) PRIMARY KEY)",
            Queries::InsertRemotePeer => {
                if cfg!(feature = "ssl") {
                    "INSERT IGNORE INTO remote_peer (address, public_key) VALUES (:address, :public_key)"
                } else {
                    "INSERT IGNORE INTO remote_peer (address) VALUES (:address)"
                }
            },
            Queries::InsertRequestPart => "INSERT INTO request_part (uid, request_type, start, total, content_size, content, sender) VALUES (:uid, :request_type, :start, :total, :content_size, :content, :sender)",
            Queries::InsertBlock => "INSERT IGNORE INTO block (address) VALUES (:address)",
            Queries::InsertThread => "INSERT IGNORE INTO thread (status) VALUES (false)",
            Queries::UpdateThread => "UPDATE thread SET status = :status",
            Queries::DeleteRemotePeer => "DELETE FROM remote_peer WHERE address = :address",
            Queries::DeleteRequestPart => "DELETE FROM request_part WHERE uid = :uid",
            Queries::DeleteBlock => "DELETE FROM block WHERE address = :address",
            Queries::SelectRemotePeerByAddress => "SELECT * FROM remote_peer WHERE address = :address",
            Queries::SelectAllRemotePeer => "SELECT * FROM remote_peer",
            Queries::SelectRequestPartByUid => "SELECT * FROM request_part WHERE uid = :uid",
            Queries::SelectBlock => "SELECT * FROM block",
            Queries::SelectThread => "SELECT * FROM thread",
            Queries::DropTableRemotePeer => "DROP TABLE IF EXISTS remote_peer",
            Queries::DropTableRequestPart => "DROP TABLE IF EXISTS request_part",
            Queries::DropTableThread => "DROP TABLE IF EXISTS thread",
            Queries::DropTableBlock => "DROP TABLE IF EXISTS block",
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

async fn execute(pool: &Pool, query: Queries, params: Params) -> usize {
    let mut connection = get_connection(pool).await;
    let res = connection
        .exec_iter(query.sql(), params.clone())
        .or_else(|e| {
            log::error!("{e}");
            Err(e)
        })
        .unwrap();
    res.affected_rows() as usize
}

async fn prepare<T>(
    pool: &Pool,
    query: Queries,
    params: Params,
    mapper: impl Fn(&Row) -> T,
) -> Vec<T> {
    let mut connection = get_connection(pool).await;
    let res = connection
        .exec_iter(query.sql(), params.clone())
        .or_else(|e| {
            log::error!("{e}");
            Err(e)
        })
        .unwrap();
    let mut result = Vec::new();
    for row in res {
        let row = row
            .or_else(|e| {
                log::error!("{e}");
                Err(e)
            })
            .unwrap();
        let value = mapper(&row);
        result.push(value)
    }
    result
}

#[async_trait]
impl PeerDao for MysqlPeerDao {
    async fn init(&mut self) -> () {
        execute(&self.pool, Queries::DropTableRemotePeer, Params::Empty).await;
        execute(&self.pool, Queries::DropTableRequestPart, Params::Empty).await;
        execute(&self.pool, Queries::DropTableBlock, Params::Empty).await;
        execute(&self.pool, Queries::DropTableThread, Params::Empty).await;
        execute(&self.pool, Queries::CreateTableRemotePeer, Params::Empty).await;
        execute(&self.pool, Queries::CreateTableRequestPart, Params::Empty).await;
        execute(&self.pool, Queries::CreateTableBlock, Params::Empty).await;
        execute(&self.pool, Queries::CreateTableThread, Params::Empty).await;
        execute(&self.pool, Queries::InsertThread, Params::Empty).await;
    }

    #[cfg(feature = "ssl")]
    async fn add_remote(&self, address: &SocketAddr, public_key: &Vec<u8>) -> RemotePeer {
        execute(
            &self.pool,
            Queries::InsertRemotePeer,
            params! { "address" => address.to_string(), "public_key" => public_key },
        )
        .await;
        RemotePeer::new(address, public_key)
    }
    #[cfg(not(feature = "ssl"))]
    async fn add_remote(&self, address: &SocketAddr) -> RemotePeer {
        execute(
            &self.pool,
            Queries::InsertRemotePeer,
            params! { "address" => address.to_string() },
        )
        .await;
        RemotePeer::new(address)
    }

    async fn remove_remote(&self, remote: &RemotePeer) -> usize {
        execute(
            &self.pool,
            Queries::DeleteRemotePeer,
            params! { "address" => remote.addr.to_string() },
        )
        .await
    }

    async fn find_remotes_by_address(&self, address: &SocketAddr) -> Vec<RemotePeer> {
        prepare(
            &self.pool,
            Queries::SelectRemotePeerByAddress,
            params! { "address" => address.to_string() },
            remote_peer_mapper,
        )
        .await
    }

    async fn find_all_remotes(&self) -> Vec<RemotePeer> {
        prepare(
            &self.pool,
            Queries::SelectAllRemotePeer,
            Params::Empty,
            remote_peer_mapper,
        )
        .await
    }

    async fn add_request_part(&self, part: &RequestPart) -> usize {
        execute(
            &self.pool,
            Queries::InsertRequestPart,
            params! {
                "uid" => part.uid.clone(),
                "request_type" => part.request_type.to_code(),
                "start" => part.start,
                "total" => part.total,
                "content_size" => part.content_size,
                "content" => part.content.clone(),
                "sender" => part.sender.to_string(),
            },
        )
        .await
    }

    async fn remove_request_part_by_uid(&self, uid: &String) -> usize {
        execute(
            &self.pool,
            Queries::DeleteRequestPart,
            params! {"uid" => uid},
        )
        .await
    }

    async fn find_requests_part_by_uid(&self, uid: &String) -> Vec<RequestPart> {
        prepare(
            &self.pool,
            Queries::SelectRequestPartByUid,
            params! {"uid" => uid},
            request_part_mapper,
        )
        .await
    }

    async fn block(&self, address: &SocketAddr) -> usize {
        execute(
            &self.pool,
            Queries::InsertBlock,
            params! { "address" => address.to_string() },
        )
        .await
    }

    async fn unblock(&self, address: &SocketAddr) -> usize {
        execute(
            &self.pool,
            Queries::DeleteBlock,
            params! { "address" => address.to_string() },
        )
        .await
    }

    async fn find_all_block(&self) -> Vec<SocketAddr> {
        prepare(
            &self.pool,
            Queries::SelectBlock,
            Params::Empty,
            block_mapper,
        )
        .await
    }

    async fn update_status(&self, value: bool) -> () {
        execute(
            &self.pool,
            Queries::UpdateThread,
            params! { "status" => value },
        )
        .await;
    }

    async fn find_status(&self) -> bool {
        let lines = prepare(
            &self.pool,
            Queries::SelectThread,
            Params::Empty,
            thread_mapper,
        )
        .await;
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
