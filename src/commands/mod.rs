mod common;
mod handler;
mod menu;
pub mod state;

pub trait CommandConfigure<'a> {
    fn setup_commands(self) -> Self;
}
