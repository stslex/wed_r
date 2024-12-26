use teloxide::{dispatching::DpHandlerDescription, dptree::Handler, prelude::DependencyMap};

use super::{
    callbacks::CallbacksHandler, common_commands::CommonCommandsHandler, menu::MenuCommandsHandler,
    HandlerConfigure,
};

impl<'a: 'static> HandlerConfigure<'a>
    for Handler<
        'a,
        DependencyMap,
        Result<(), Box<dyn std::error::Error + Send + Sync>>,
        DpHandlerDescription,
    >
{
    fn setup_handlers(self) -> Self {
        self.setup_callbacks()
            .setup_main_commands()
            .setup_common_commands()
    }
}
