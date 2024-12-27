use diesel::{
    r2d2::{self},
    PgConnection,
};
pub mod database;

pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct BotState {
    pub pool: DbPool,
}

pub fn get_bot_state() -> BotState {
    BotState {
        pool: database::pool::create_db_pool(),
    }
}

#[cfg(test)]
pub fn get_test_bot_state() -> BotState {
    BotState {
        pool: database::pool::create_test_db_pool(),
    }
}
