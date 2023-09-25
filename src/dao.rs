use std::time::Duration;

pub(crate) mod block;
pub(crate) mod part;
pub(crate) mod remote;
pub(crate) mod thread;

pub(crate) type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;
pub(crate) type Connection = r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>;

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

pub(crate) async fn create_or_upgrade_db(pool: &Pool) {
    let connection = get_connection(pool).await;
    remote::create_or_upgrade(&connection).await;
    part::create_or_upgrade(&connection).await;
    block::create_or_upgrade(&connection).await;
    thread::create_or_upgrade(&connection).await;
}
