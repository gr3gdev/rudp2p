pub(crate) mod block;
pub(crate) mod part;
pub(crate) mod remote;
pub(crate) mod thread;

pub(crate) type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;
pub(crate) type Connection = r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>;

pub(crate) async fn get_connection(pool: &Pool) -> Connection {
    let pool = pool.clone();
    pool.get().expect("Unable to obtain pool")
}

pub(crate) async fn create_or_upgrade_db(pool: &Pool) {
    let connection = get_connection(pool).await;
    remote::create_or_upgrade(&connection).await;
    part::create_or_upgrade(&connection).await;
    block::create_or_upgrade(&connection).await;
    thread::create_or_upgrade(&connection).await;
}
