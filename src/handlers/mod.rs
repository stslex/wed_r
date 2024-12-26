mod callbacks;
mod common_commands;
mod handler;
mod menu;
pub mod state;

pub trait HandlerConfigure<'a> {
    fn setup_handlers(self) -> Self;
}
