use super::*;
use crate::{
    configuration::DatabaseUpgradeMode,
    network::request::Type,
    utils::{decoder::Decoder, encoder::Encoder},
};
use std::{cmp::Ordering, fmt::Debug, net::SocketAddr};

#[cfg(feature = "mysql")]
use mysql::{params, Result, Row};
#[cfg(feature = "sqlite")]
use rusqlite::{Result, Row};

#[derive(Clone, Eq, PartialEq, Ord)]
pub(crate) struct RequestPart {
    pub(crate) uid: String,
    pub(crate) request_type: Type,
    pub(crate) start: usize,
    pub(crate) total: usize,
    pub(crate) content_size: usize,
    pub(crate) content: Vec<u8>,
    pub(crate) sender: SocketAddr,
}

impl Debug for RequestPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RequestPart")
            .field("uid", &self.uid)
            .field("request_type", &self.request_type)
            .field("start", &self.start)
            .field("total", &self.total)
            .field("content_size", &self.content_size)
            .field("sender", &self.sender)
            .finish()
    }
}

impl RequestPart {
    pub(crate) fn to_data(&self) -> Vec<u8> {
        let mut data = Vec::new();
        data.push(self.request_type.to_code());
        let data = Encoder::add_with_size(&data, &self.uid.as_bytes().to_vec());
        let data = Encoder::add_size(&data, self.start);
        let data = Encoder::add_size(&data, self.total);
        let mut data = Encoder::add_size(&data, self.content_size);
        data.append(&mut self.content.clone());
        log::trace!("RequestPart::to_data() => {}", data.len());
        data
    }

    pub(crate) fn parse(data: Vec<u8>, addr: SocketAddr) -> Self {
        let request_type = Type::from_code(data[0]);
        let (uid_size, next_index) = Decoder::get_size(&data, 1);
        let uid = unwrap_result(
            String::from_utf8(data[next_index..next_index + uid_size].to_vec()),
            "Unable to read the UID",
        );
        let (start, next_index) = Decoder::get_size(&data, next_index + uid_size);
        let (total, next_index) = Decoder::get_size(&data, next_index);
        let (content_size, next_index) = Decoder::get_size(&data, next_index);
        let res = Self {
            uid,
            request_type,
            start,
            total,
            content_size,
            content: data[next_index..].to_vec(),
            sender: addr,
        };
        log::trace!("RequestPart::parse({}, {addr}) => {:?}", data.len(), res);
        res
    }
}

impl PartialOrd for RequestPart {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.start < other.start {
            Some(Ordering::Less)
        } else if self.start > other.start {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }
}

enum Queries {
    Drop,
    CreateOrUpgrade,
    SelectByUid,
    Add,
    RemoveByUid,
}

impl ToSql for Queries {
    fn to_sql(&self) -> &str {
        match self {
            Queries::Drop => "DROP TABLE IF EXISTS request_part",
            Queries::CreateOrUpgrade => {
                if cfg!(feature = "mysql") {
                    "CREATE TABLE IF NOT EXISTS request_part (
                        id INTEGER NOT NULL AUTO_INCREMENT,
                        uid VARCHAR(24),
                        type INTEGER,
                        start INTEGER,
                        total INTEGER,
                        content_size INTEGER,
                        content BLOB(1024),
                        sender VARCHAR(24),
                        PRIMARY KEY (id)
                    )"
                } else {
                    "CREATE TABLE IF NOT EXISTS request_part (
                        id INTEGER PRIMARY KEY,
                        uid TEXT,
                        type INTEGER,
                        start INTEGER,
                        total INTEGER,
                        content_size INTEGER,
                        content TEXT,
                        sender TEXT
                    )"
                }
            }
            Queries::SelectByUid => "SELECT DISTINCT uid, type, start, total, content_size, content, sender FROM request_part WHERE uid = :uid",
            Queries::Add => "INSERT INTO request_part (uid, type, start, total, content_size, content, sender) VALUES (:uid, :type, :start, :total, :content_size, :content, :sender)",
            Queries::RemoveByUid => "DELETE FROM request_part WHERE uid = :uid",
        }
    }
}

#[cfg(feature = "sqlite")]
fn mapper(row: &Row) -> Result<RequestPart> {
    let uid = unwrap_result(row.get(0), "Unable to read 'uid'");
    let request_type = unwrap_result(row.get(1), "Unable to read 'type'");
    let start = unwrap_result(row.get(2), "Unable to read 'start'");
    let total = unwrap_result(row.get(3), "Unable to read 'total'");
    let content_size = unwrap_result(row.get(4), "Unable to read 'content_size'");
    let content = unwrap_result(row.get(5), "Unable to read 'content'");
    let sender: String = unwrap_result(row.get(6), "Unable to read 'sender'");
    Ok(RequestPart {
        uid,
        request_type,
        start,
        total,
        content_size,
        content,
        sender: unwrap_result(sender.parse(), "Unable to parse address !"),
    })
}

#[cfg(feature = "mysql")]
fn mapper(row: &Row) -> Result<RequestPart> {
    let uid = unwrap_option(row.get(0), "Unable to read 'uid'");
    let request_type = unwrap_option(row.get(1), "Unable to read 'type'");
    let start = unwrap_option(row.get(2), "Unable to read 'start'");
    let total = unwrap_option(row.get(3), "Unable to read 'total'");
    let content_size = unwrap_option(row.get(4), "Unable to read 'content_size'");
    let content = unwrap_option(row.get(5), "Unable to read 'content'");
    let sender: String = unwrap_option(row.get(6), "Unable to read 'sender'");
    Ok(RequestPart {
        uid,
        request_type,
        start,
        total,
        content_size,
        content,
        sender: unwrap_result(sender.parse(), "Unable to parse address !"),
    })
}

pub(crate) async fn create_or_upgrade(pool: &Pool, database_upgrade_mode: &DatabaseUpgradeMode) {
    match database_upgrade_mode {
        DatabaseUpgradeMode::Upgrade => execute(pool, Queries::CreateOrUpgrade, EMPTY).await,
        DatabaseUpgradeMode::AlwaysNew => {
            execute(pool, Queries::Drop, EMPTY).await;
            execute(pool, Queries::CreateOrUpgrade, EMPTY).await
        }
    };
}

#[cfg(feature = "sqlite")]
pub(crate) async fn select_by_uid(pool: &Pool, uid: &String) -> Vec<RequestPart> {
    prepare(
        pool,
        Queries::SelectByUid,
        rusqlite::named_params! {":uid": uid},
        mapper,
    )
    .await
}

#[cfg(feature = "mysql")]
pub(crate) async fn select_by_uid(pool: &Pool, uid: &String) -> Vec<RequestPart> {
    prepare(
        pool,
        Queries::SelectByUid,
        mysql::params! {"uid" => uid},
        mapper,
    )
    .await
}

#[cfg(feature = "sqlite")]
pub(crate) async fn add(pool: &Pool, part: &RequestPart) -> usize {
    execute(
        pool,
        Queries::Add,
        rusqlite::named_params! {
            ":uid": part.uid.clone(),
            ":type": part.request_type.clone(),
            ":start": part.start,
            ":total": part.total,
            ":content_size": part.content_size,
            ":content": part.content.clone(),
            ":sender": part.sender.to_string(),
        },
    )
    .await
}

#[cfg(feature = "mysql")]
pub(crate) async fn add(pool: &Pool, part: &RequestPart) -> usize {
    execute(
        pool,
        Queries::Add,
        mysql::params! {
            "uid" => part.uid.clone(),
            "type" => part.request_type.clone(),
            "start" => part.start,
            "total" => part.total,
            "content_size" => part.content_size,
            "content" => part.content.clone(),
            "sender" => part.sender.to_string(),
        },
    )
    .await
}

#[cfg(feature = "sqlite")]
pub(crate) async fn remove_by_uid(pool: &Pool, uid: &String) -> usize {
    execute(
        pool,
        Queries::RemoveByUid,
        rusqlite::named_params! {":uid": uid},
    )
    .await
}

#[cfg(feature = "mysql")]
pub(crate) async fn remove_by_uid(pool: &Pool, uid: &String) -> usize {
    execute(pool, Queries::RemoveByUid, mysql::params! {"uid" => uid}).await
}

#[cfg(test)]
mod tests {
    use super::RequestPart;
    use crate::{configuration::Configuration, dao, network, utils::generate_uid};
    use futures::executor::block_on;

    #[test]
    fn parse() {
        let content = "This is a content for test".as_bytes().to_vec();
        let address = "127.0.0.1:9999".parse().unwrap();
        let part = RequestPart {
            uid: String::from("PART4TEST"),
            request_type: network::request::Type::Connection,
            start: 0,
            total: 256,
            content_size: content.len(),
            content,
            sender: address,
        };
        let data = part.to_data();
        let parse = RequestPart::parse(data, address);
        assert_eq!(part, parse);
    }

    #[cfg(all(feature = "mysql", not(feature = "ssl")))]
    fn init_configuration() -> Configuration {
        Configuration::builder()
            .database("mysql://cucumber:test@localhost:3388/tests")
            .build()
    }

    #[cfg(all(feature = "mysql", feature = "ssl"))]
    fn init_configuration() -> Configuration {
        use crate::configuration::SSL;

        Configuration::builder(SSL::from_size(1024))
            .database("mysql://cucumber:test@localhost:3388/tests")
            .build()
    }

    #[cfg(all(feature = "sqlite", not(feature = "ssl")))]
    fn init_configuration() -> Configuration {
        Configuration::builder().build()
    }

    #[cfg(all(feature = "sqlite", feature = "ssl"))]
    fn init_configuration() -> Configuration {
        use crate::configuration::SSL;

        Configuration::builder(SSL::from_size(1024)).build()
    }

    #[test]
    fn add_and_remove_part() {
        block_on(async {
            let configuration = init_configuration();
            let pool = dao::init(&configuration).await;
            let text = String::from("this is a test message");
            let uid = generate_uid("P");
            let part = dao::part::RequestPart {
                uid: uid.clone(),
                request_type: network::request::Type::Message,
                start: 0,
                total: text.len(),
                content_size: text.len(),
                content: text.as_bytes().to_vec(),
                sender: "127.0.0.1:9000".parse().unwrap(),
            };
            dao::part::add(&pool, &part).await;
            let parts = dao::part::select_by_uid(&pool, &uid).await;
            assert!(parts.len() == 1);
            assert_eq!(&part, parts.get(0).unwrap());
            let nb = dao::part::remove_by_uid(&pool, &uid).await;
            assert_eq!(1, nb);
        });
    }
}
