use callbacks::CallbacksHandler;
use common_commands::CommonCommandsHandler;
use menu::MenuCommandsHandler;
use teloxide::{dispatching::UpdateHandler, dptree};

mod callbacks;
mod common_commands;
mod menu;
pub mod state;

pub fn handler_tree() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    dptree::entry()
        .setup_callbacks()
        .setup_main_commands()
        .setup_common_commands()
}
