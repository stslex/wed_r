use diesel::{r2d2, PgConnection};

use crate::config::DbPool;

use super::DbMigration;

pub fn create_db_pool() -> DbPool {
    let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL should be set");
    let manager = r2d2::ConnectionManager::<PgConnection>::new(conn_spec);

    r2d2::Pool::builder()
        .build(manager)
        .expect("database URL should be valid path to SQLite DB file")
        .run_migrations()
}

#[cfg(test)]
pub fn create_test_db_pool() -> DbPool {
    use diesel::Connection;

    let database_url: &str = "postgres://postgres:postgres@localhost:5432/postgres";
    let manager = r2d2::ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("database URL should be valid path to SQLite DB file");
    pool.get().unwrap().begin_test_transaction().unwrap();
    pool.run_migrations()
}
