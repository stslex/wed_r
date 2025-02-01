use diesel::{
    r2d2::{self, ConnectionManager, PooledConnection},
    PgConnection,
};
use serde::{Deserialize, Serialize};

use crate::repository::admin::model::UserResponseModel;
pub mod database;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbCon = PooledConnection<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct BotState {
    pub pool: DbPool,
}

impl BotState {
    pub fn new() -> BotState {
        BotState {
            pool: database::pool::create_db_pool(),
        }
    }

    #[cfg(test)]
    pub fn new_test() -> BotState {
        BotState {
            pool: database::pool::create_test_db_pool(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub enum AdminDialogue {
    #[default]
    None,
    CreateUser(CreateUserState),
    RemoveUser(RemoveUserState),
}

#[derive(Clone, Serialize, Deserialize)]
pub enum CreateUserState {
    WaitingForUsername,
    WaitingForAccept { firstname: String },
}

#[derive(Clone, Serialize, Deserialize)]
pub enum RemoveUserState {
    WaitingForNumber { users: Vec<UserResponseModel> },
    WaitingForAccept { user: UserResponseModel },
}
