use diesel::PgConnection;
use log::error;

use crate::{
    config::{BotState, DbCon},
    routes::model::ErrorResponseData,
};

use super::DataPool;

impl DataPool<PgConnection, ErrorResponseData> for BotState {
    fn safe_get(self) -> Result<DbCon, ErrorResponseData> {
        self.pool.get().map_err(|e| {
            error!("Failed to get connection from pool: {}", e);
            ErrorResponseData::InternalServerError
        })
    }
}
