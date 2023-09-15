use log::{debug, error};
use std::{cmp::Ordering, fmt::Debug};

use crate::{
    network::request::Type,
    utils::{decoder::Decoder, encoder::Encoder},
};

use super::*;

#[derive(Clone, Eq, PartialEq, Ord)]
pub(crate) struct RequestPart {
    pub(crate) uid: String,
    pub(crate) request_type: Type,
    pub(crate) start: usize,
    pub(crate) total: usize,
    pub(crate) content_size: usize,
    pub(crate) content: Vec<u8>,
}

impl Debug for RequestPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RequestPart")
            .field("uid", &self.uid)
            .field("request_type", &self.request_type)
            .field("start", &self.start)
            .field("total", &self.total)
            .field("content_size", &self.content_size)
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

    pub(crate) fn parse(data: Vec<u8>) -> Self {
        let request_type = Type::from_code(data[0]);
        let (uid_size, next_index) = Decoder::get_size(&data, 1);
        let uid =
            String::from_utf8(data[next_index..next_index + uid_size].to_vec()).expect("Unable to read the UID");
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

pub(crate) async fn create_or_upgrade(connection: &Connection) {
    connection
        .execute(
            "
    CREATE TABLE IF NOT EXISTS request_part (
        id INTEGER PRIMARY KEY,
        uid TEXT,
        type INTEGER,
        start INTEGER,
        total INTEGER,
        content_size INTEGER,
        content TEXT
    )",
            [],
        )
        .expect("Unable to create table 'request_part'");
}

fn mapper(row: &rusqlite::Row<'_>) -> RequestPart {
    let uid = row.get(0).expect("Unable to read 'uid'");
    let request_type = row.get(1).expect("Unable to read 'type'");
    let start = row.get(2).expect("Unable to read 'start'");
    let total = row.get(3).expect("Unable to read 'total'");
    let content_size = row.get(4).expect("Unable to read 'content_size'");
    let content = row.get(5).expect("Unable to read 'content'");
    RequestPart {
        uid,
        request_type,
        start,
        total,
        content_size,
        content,
    }
}

pub(crate) async fn select_by_uid(pool: &Pool, uid: &String) -> Vec<RequestPart> {
    let connection = get_connection(pool).await;
    let mut statement = connection
        .prepare("SELECT uid, type, start, total, content_size, content FROM request_part WHERE uid = ?1")
        .expect("Unable to prepare query : select_by_uid");
    statement
        .query_map([uid], |row| {
            let part = mapper(row);
            debug!("select_by_uid({}) = {:?}", uid, part);
            Ok(part)
        })
        .and_then(Iterator::collect)
        .unwrap_or_else(|e| {
            error!("{e}");
            vec![]
        })
}

pub(crate) async fn add(pool: &Pool, part: &RequestPart) -> usize {
    let connection = get_connection(pool).await;
    connection
        .execute(
            "INSERT INTO request_part (uid, type, start, total, content_size, content) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            (part.uid.clone(), part.request_type.clone(), part.start, part.total, part.content_size, part.content.clone()),
        )
        .and_then(|nb| {
            debug!("add({:?}) = {}", part, nb);
            Ok(nb)
        })
        .unwrap_or_else(|e| {
            error!("{e}");
            0
        })
}

pub(crate) async fn remove_by_uid(pool: &Pool, uid: &String) -> usize {
    let connection = get_connection(pool).await;
    connection
        .execute("DELETE FROM request_part WHERE uid = ?1", [uid])
        .and_then(|nb| {
            debug!("remove_by_uid({}) = {}", uid, nb);
            Ok(nb)
        })
        .unwrap_or_else(|e| {
            error!("{e}");
            0
        })
}

#[cfg(test)]
mod tests {
    use crate::network::request::Type;

    use super::RequestPart;

    #[test]
    fn parse() {
        let content = "This is a content for test".as_bytes().to_vec();
        let part = RequestPart {
            uid: String::from("PART4TEST"),
            request_type: Type::Connection,
            start: 0,
            total: 256,
            content_size: content.len(),
            content,
        };
        let data = part.to_data();
        let parse = RequestPart::parse(data);
        assert_eq!(part, parse);
    }
}
