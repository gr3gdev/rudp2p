use crate::configuration::Configuration;
use std::time::Duration;

pub(crate) mod block;
pub(crate) mod part;
pub(crate) mod remote;
pub(crate) mod thread;

#[cfg(feature = "sqlite")]
pub(crate) type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;
#[cfg(feature = "sqlite")]
pub(crate) type Connection = r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>;

#[cfg(feature = "mysql")]
pub(crate) type Pool = r2d2::Pool<r2d2_mysql::MySqlConnectionManager>;
#[cfg(feature = "mysql")]
pub(crate) type Connection = r2d2::PooledConnection<r2d2_mysql::MySqlConnectionManager>;

pub(crate) async fn get_connection(pool: &Pool) -> Connection {
    let pool = pool.clone();
    let mut connection = pool.try_get();
    if connection.is_none() {
        let mut count = 1;
        while connection.is_none() && count < 5 {
            connection = pool.try_get();
            std::thread::sleep(Duration::from_millis(count * 100));
            count = count + 1;
        }
    }
    connection.expect("Unable to get a connection")
}

async fn create_or_upgrade_db(pool: &Pool) {
    remote::create_or_upgrade(pool).await;
    part::create_or_upgrade(pool).await;
    block::create_or_upgrade(pool).await;
    thread::create_or_upgrade(pool).await;
}

#[cfg(feature = "sqlite")]
pub(crate) async fn init(configuration: &Configuration) -> Pool {
    let manager = match configuration.database_mode.clone() {
        crate::configuration::SqliteMode::Memory => r2d2_sqlite::SqliteConnectionManager::memory(),
        crate::configuration::SqliteMode::File(path) => {
            r2d2_sqlite::SqliteConnectionManager::file(&path)
        }
    }
    .with_init(|c| {
        c.execute_batch(
            "PRAGMA journal_mode=wal2; PRAGMA synchronous=NORMAL; PRAGMA foreign_keys=1;",
        )
    });
    let pool = Pool::builder()
        .max_size(16)
        .build(manager)
        .expect("Unable to initialize pool");
    create_or_upgrade_db(&pool).await;
    pool
}

#[cfg(feature = "mysql")]
pub(crate) async fn init(configuration: &Configuration) -> Pool {
    let opts = mysql::Opts::from_url(&configuration.database_url).expect("Invalid database url");
    let params = mysql::OptsBuilder::from_opts(opts);
    let manager = r2d2_mysql::MySqlConnectionManager::new(params);
    let pool = Pool::builder()
        .max_size(16)
        .build(manager)
        .expect("Unable to initialize pool");
    create_or_upgrade_db(&pool).await;
    pool
}

#[cfg(feature = "sqlite")]
pub(crate) async fn execute<P: rusqlite::Params>(pool: &Pool, sql: &str, params: P) -> () {
    let connection = get_connection(pool).await;
    connection
        .execute(sql, params)
        .expect(format!("Unable to execute : {}", sql).as_str());
}

#[cfg(feature = "mysql")]
pub(crate) async fn execute<P: Into<mysql::Params>>(pool: &Pool, sql: &str, params: P) -> () {
    use mysql::prelude::Queryable;

    let mut connection = get_connection(pool).await;
    connection
        .exec_drop(sql, params)
        .expect(format!("Unable to execute : {}", sql).as_str())
}

#[cfg(feature = "sqlite")]
pub(crate) async fn prepare<P, F, T>(pool: &Pool, sql: &str, params: P, f: F) -> Vec<T>
where
    P: rusqlite::Params,
    F: FnMut(&rusqlite::Row<'_>) -> rusqlite::Result<T>,
{
    let connection = get_connection(pool).await;
    let mut statement = connection
        .prepare(sql)
        .expect(format!("Unable to prepare : {}", sql).as_str());
    statement
        .query_map(params, f)
        .and_then(Iterator::collect)
        .expect(format!("Unable to query : {}", sql).as_str())
}

#[cfg(feature = "mysql")]
pub(crate) async fn prepare<P, F, T>(pool: &Pool, sql: &str, params: P, mut f: F) -> Vec<T>
where
    P: Into<mysql::Params>,
    F: FnMut(mysql::Row) -> mysql::Result<T>,
{
    use mysql::prelude::Queryable;

    let mut connection = get_connection(pool).await;
    let res = connection
        .exec_iter(sql, params)
        .expect(format!("Unable to query : {}", sql).as_str());
    let mut result = Vec::new();
    for row in res {
        result.push(f(row.unwrap()).unwrap())
    }
    result
}
