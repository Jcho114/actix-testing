use crate::core::config::CONFIG;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use secrecy::ExposeSecret;

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;

pub fn create_sqlite_pool() -> DbPool {
    let db_path = CONFIG.db_path.expose_secret();
    let manager = ConnectionManager::<SqliteConnection>::new(db_path);
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}
