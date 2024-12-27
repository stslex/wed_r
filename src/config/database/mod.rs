mod migration;
pub mod pool;

trait DbMigration {
    fn run_migrations(&self) -> Self;
}
