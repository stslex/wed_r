use crate::config::DbPool;

use super::DbMigration;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

impl DbMigration for DbPool {
    fn run_migrations(&self) -> Self {
        match self.get().unwrap().run_pending_migrations(MIGRATIONS) {
            Ok(res) => {
                log::info!("Migrations are run successfully {:?}", res);
                self.clone()
            }
            Err(e) => panic!("Failed to run migrations: {}", e),
        };
        self.clone()
    }
}

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");
