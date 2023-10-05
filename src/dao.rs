use crate::configuration::Configuration;
use std::time::Duration;

#[cfg(feature = "mysql")]
use mysql::{Params, Result, Row};
#[cfg(feature = "sqlite")]
use rusqlite::{Params, Result, Row};

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

pub(crate) trait ToSql {
    fn to_sql(&self) -> &str;
}

#[cfg(feature = "sqlite")]
pub(crate) const EMPTY: &[&dyn rusqlite::ToSql] = rusqlite::params![];
#[cfg(feature = "mysql")]
pub(crate) const EMPTY: Params = Params::Empty;

/// Get a generic connection (compatible "sqlite" and "mysql")
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
    log::trace!("create_or_upgrade_db({:?})", pool);
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
    log::trace!("init({:?}) => {:?}", configuration, pool);
    pool
}

#[cfg(feature = "mysql")]
pub(crate) async fn init(configuration: &Configuration) -> Pool {
    if let Some(url) = &configuration.database_url {
        let opts = mysql::Opts::from_url(url)
            .expect(format!("Error when parsing {:?}", configuration).as_str());
        let params = mysql::OptsBuilder::from_opts(opts);
        let manager = r2d2_mysql::MySqlConnectionManager::new(params);
        let pool = Pool::builder()
            .max_size(16)
            .build(manager)
            .expect("Unable to initialize pool");
        create_or_upgrade_db(&pool).await;
        log::trace!("init({:?}) => {:?}", configuration, pool);
        pool
    } else {
        log::error!("Error in configuration : {:?}", configuration);
        panic!("Missing databse url !")
    }
}

#[cfg(feature = "sqlite")]
pub(crate) async fn execute<S, P>(pool: &Pool, sql: S, params: P) -> usize
where
    S: ToSql,
    P: Params,
{
    let sql = sql.to_sql();
    let connection = get_connection(pool).await;
    let nb_updates = connection
        .execute(sql, params)
        .expect(format!("Unable to execute : {sql}").as_str());
    log::trace!("execute(pool, {sql}, params) => {nb_updates}");
    nb_updates
}

#[cfg(feature = "mysql")]
pub(crate) async fn execute<S: ToSql>(pool: &Pool, sql: S, params: Params) -> usize {
    use mysql::prelude::Queryable;

    let sql = sql.to_sql();
    let mut connection = get_connection(pool).await;
    let res = connection.exec_iter(sql, params.clone());
    log::trace!("execute(pool, {sql}, {:?}) => {:?}", params, res);
    let res = res.expect(format!("Unable to execute : {sql}").as_str());
    res.affected_rows() as usize
}

#[cfg(feature = "sqlite")]
pub(crate) async fn prepare<S, P, F, T>(pool: &Pool, sql: S, params: P, f: F) -> Vec<T>
where
    S: ToSql,
    P: Params,
    F: Fn(&Row) -> Result<T>,
    T: std::fmt::Debug,
{
    let sql = sql.to_sql();
    let connection = get_connection(pool).await;
    let mut statement = connection
        .prepare(sql)
        .expect(format!("Unable to prepare : {sql}").as_str());
    let res = statement
        .query_map(params, |row| f(row))
        .and_then(Iterator::collect)
        .expect(format!("Unable to query : {sql}").as_str());
    log::trace!("prepare(pool, {sql}, params, mapper) => {:?}", res);
    res
}

#[cfg(feature = "mysql")]
pub(crate) async fn prepare<S, F, T>(pool: &Pool, sql: S, params: Params, mapper: F) -> Vec<T>
where
    S: ToSql,
    F: Fn(&Row) -> Result<T>,
    T: std::fmt::Debug,
{
    use mysql::prelude::Queryable;

    let sql = sql.to_sql();
    let mut connection = get_connection(pool).await;
    let res = connection
        .exec_iter(sql, params.clone())
        .expect(format!("Unable to query : {sql}").as_str());
    let mut result = Vec::new();
    for row in res {
        let row = row.expect("Unable to read row");
        let value = mapper(&row).expect(&format!("Unable to map row {:?}", row));
        result.push(value)
    }
    log::trace!("prepare(pool, {sql}, {:?}, mapper) => {:?}", params, result,);
    result
}
