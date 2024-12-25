use teloxide::{dispatching::DpHandlerDescription, dptree::Handler, prelude::DependencyMap};

use super::{common::CommonCommands, menu::MenuCommands, CommandConfigure};

impl<'a: 'static> CommandConfigure<'a>
    for Handler<
        'a,
        DependencyMap,
        Result<(), Box<dyn std::error::Error + Send + Sync>>,
        DpHandlerDescription,
    >
{
    fn setup_commands(self) -> Self {
        self.setup_main_commands().setup_common_commands()
    }
}
