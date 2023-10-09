#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Type {
    Connection,
    Disconnection,
    Message,
    ShareConnection,
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

#[cfg(feature = "sqlite")]
mod sqlite {
    use crate::utils::unwrap::unwrap_result;

    use super::Type;
    use rusqlite::{types::FromSql, ToSql};

    impl FromSql for Type {
        fn column_result(
            value: rusqlite::types::ValueRef<'_>,
        ) -> rusqlite::types::FromSqlResult<Self> {
            rusqlite::types::FromSqlResult::Ok(Type::from_code(unwrap_result(
                value.as_i64(),
                "Unable to read Type value",
            ) as u8))
        }
    }

    impl ToSql for Type {
        fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
            Ok(rusqlite::types::ToSqlOutput::from(self.to_code()))
        }
    }
}

#[cfg(feature = "mysql")]
mod sqlite {
    use super::Type;
    use mysql::{prelude::FromValue, FromValueError};

    pub struct ParseType {
        code: i64,
    }

    impl TryFrom<mysql::Value> for ParseType {
        type Error = FromValueError;

        fn try_from(value: mysql::Value) -> Result<Self, Self::Error> {
            match value {
                mysql::Value::Int(v) => Ok(Self { code: v }),
                _ => Err(FromValueError(value)),
            }
        }
    }

    impl From<ParseType> for Type {
        fn from(value: ParseType) -> Self {
            Type::from_code(value.code as u8)
        }
    }

    impl FromValue for Type {
        type Intermediate = ParseType;
    }

    impl From<Type> for mysql::Value {
        fn from(value: Type) -> Self {
            mysql::Value::Int(value.to_code() as i64)
        }
    }
}
