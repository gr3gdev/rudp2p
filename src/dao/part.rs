use super::*;
use crate::{
    network::request::Type,
    utils::{decoder::Decoder, encoder::Encoder},
};
use std::{cmp::Ordering, fmt::Debug, net::SocketAddr};

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
        let data = Encoder::add_with_size(data, self.uid.as_bytes().to_vec());
        let data = Encoder::add_size(data, self.start);
        let data = Encoder::add_size(data, self.total);
        let mut data = Encoder::add_size(data, self.content_size);
        data.append(&mut self.content.clone());
        data
    }

    pub(crate) fn parse(data: Vec<u8>, addr: SocketAddr) -> Self {
        let request_type = Type::from_code(data[0]);
        let (uid_size, next_index) = Decoder::get_size(&data, 1);
        let uid = String::from_utf8(data[next_index..next_index + uid_size].to_vec())
            .expect("Unable to read the UID");
        let (start, next_index) = Decoder::get_size(&data, next_index + uid_size);
        let (total, next_index) = Decoder::get_size(&data, next_index);
        let (content_size, next_index) = Decoder::get_size(&data, next_index);
        Self {
            uid,
            request_type,
            start,
            total,
            content_size,
            content: data[next_index..].to_vec(),
            sender: addr,
        }
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

#[cfg(feature = "sqlite")]
pub(crate) async fn create_or_upgrade(pool: &Pool) -> () {
    let sql = "
    CREATE TABLE IF NOT EXISTS request_part (
        id INTEGER PRIMARY KEY,
        uid TEXT,
        type INTEGER,
        start INTEGER,
        total INTEGER,
        content_size INTEGER,
        content TEXT,
        sender TEXT
    )";
    execute(pool, sql, {}).await;
}

#[cfg(feature = "mysql")]
pub(crate) async fn create_or_upgrade(pool: &Pool) {
    let sql = "
    CREATE TABLE IF NOT EXISTS request_part (
        id INTEGER NOT NULL AUTO_INCREMENT,
        uid TEXT,
        type INTEGER,
        start INTEGER,
        total INTEGER,
        content_size INTEGER,
        content TEXT,
        sender TEXT,
        PRIMARY KEY (id)
    )";
    execute(pool, sql, {}).await;
}

#[cfg(feature = "sqlite")]
fn mapper(row: &rusqlite::Row<'_>) -> RequestPart {
    let uid = row.get(0).expect("Unable to read 'uid'");
    let request_type = row.get(1).expect("Unable to read 'type'");
    let start = row.get(2).expect("Unable to read 'start'");
    let total = row.get(3).expect("Unable to read 'total'");
    let content_size = row.get(4).expect("Unable to read 'content_size'");
    let content = row.get(5).expect("Unable to read 'content'");
    let sender: String = row.get(6).expect("Unable to read 'sender'");
    RequestPart {
        uid,
        request_type,
        start,
        total,
        content_size,
        content,
        sender: sender.parse().unwrap(),
    }
}

#[cfg(feature = "mysql")]
fn mapper(row: mysql::Row) -> RequestPart {
    let uid = row.get(0).expect("Unable to read 'uid'");
    let request_type = row.get(1).expect("Unable to read 'type'");
    let start = row.get(2).expect("Unable to read 'start'");
    let total = row.get(3).expect("Unable to read 'total'");
    let content_size = row.get(4).expect("Unable to read 'content_size'");
    let content = row.get(5).expect("Unable to read 'content'");
    let sender: String = row.get(6).expect("Unable to read 'sender'");
    RequestPart {
        uid,
        request_type,
        start,
        total,
        content_size,
        content,
        sender: sender.parse().unwrap(),
    }
}

#[cfg(feature = "sqlite")]
pub(crate) async fn select_by_uid(pool: &Pool, uid: &String) -> Vec<RequestPart> {
    let sql = "SELECT DISTINCT uid, type, start, total, content_size, content, sender FROM request_part WHERE uid = ?1";
    prepare(pool, sql, [uid], |row| Ok(mapper(row))).await
}

#[cfg(feature = "mysql")]
pub(crate) async fn select_by_uid(pool: &Pool, uid: &String) -> Vec<RequestPart> {
    use mysql::params;

    let sql = "SELECT DISTINCT uid, type, start, total, content_size, content, sender FROM request_part WHERE uid = :uid";
    prepare(pool, sql, params! { "uid" => uid }, |row| Ok(mapper(row))).await
}

#[cfg(feature = "sqlite")]
pub(crate) async fn add(pool: &Pool, part: &RequestPart) -> usize {
    let sql = "INSERT INTO request_part (uid, type, start, total, content_size, content, sender) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)";
    execute(
        pool,
        sql,
        (
            part.uid.clone(),
            part.request_type.clone(),
            part.start,
            part.total,
            part.content_size,
            part.content.clone(),
            part.sender.to_string(),
        ),
    )
    .await
}

#[cfg(feature = "mysql")]
pub(crate) async fn add(pool: &Pool, part: &RequestPart) -> usize {
    use mysql::params;

    let sql = "INSERT INTO request_part (uid, type, start, total, content_size, content, sender) VALUES (:uid, :type, :start, :total, :content_size, :content, :sender)";
    execute(
        pool,
        sql,
        params! {
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
    let sql = "DELETE FROM request_part WHERE uid = ?1";
    execute(pool, sql, [uid]).await
}

#[cfg(feature = "mysql")]
pub(crate) async fn remove_by_uid(pool: &Pool, uid: &String) -> usize {
    use mysql::params;

    let sql = "DELETE FROM request_part WHERE uid = :uid";
    execute(pool, sql, params! { "uid" => uid }).await
}

#[cfg(test)]
mod tests {
    use super::RequestPart;
    use crate::network::request::Type;

    #[test]
    fn parse() {
        let content = "This is a content for test".as_bytes().to_vec();
        let address = "127.0.0.1:9999".parse().unwrap();
        let part = RequestPart {
            uid: String::from("PART4TEST"),
            request_type: Type::Connection,
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
}
