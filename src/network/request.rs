use rusqlite::{types::FromSql, ToSql};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Type {
    Connection,
    Disconnection,
    Message,
    ShareConnection,
}

impl FromSql for Type {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        rusqlite::types::FromSqlResult::Ok(Type::from_code(
            value.as_i64().expect("Unable to read Type value") as u8,
        ))
    }
}

impl ToSql for Type {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(rusqlite::types::ToSqlOutput::from(self.to_code()))
    }
}

impl Type {
    pub(crate) fn from_code(code: u8) -> Self {
        match code {
            0 => Self::Connection,
            1 => Self::Disconnection,
            2 => Self::Message,
            8 => Self::ShareConnection,
            _ => panic!("Code is not valid !"),
        }
    }

    pub(crate) fn to_code(&self) -> u8 {
        match self {
            Type::Connection => 0,
            Type::Disconnection => 1,
            Type::Message => 2,
            Type::ShareConnection => 8,
        }
    }
}
