use diesel::r2d2::R2D2Connection;

use super::DbCon;

mod migration;
pub mod pool;
mod utils;

trait DbMigration {
    fn run_migrations(&self) -> Self;
}

pub trait DataPool<C, E>
where
    C: R2D2Connection + 'static,
{
    fn safe_get(self) -> Result<DbCon, E>;
}
