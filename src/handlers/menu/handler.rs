use teloxide::{
    dispatching::{DpHandlerDescription, HandlerExt, UpdateFilterExt},
    dptree::{self, Handler},
    prelude::DependencyMap,
    types::Update,
};

use crate::{
    handlers::state::MenuCommandState,
    routes::{command_help, command_invite, start::command_start},
};

use super::MenuCommandsHandler;

impl<'a: 'static> MenuCommandsHandler<'a>
    for Handler<
        'a,
        DependencyMap,
        Result<(), Box<dyn std::error::Error + Send + Sync>>,
        DpHandlerDescription,
    >
{
    fn setup_main_commands(self) -> Self {
        self.branch(
            Update::filter_message().branch(
                dptree::entry()
                    .filter_command::<MenuCommandState>()
                    .branch(dptree::case![MenuCommandState::Start].endpoint(command_start))
                    .branch(dptree::case![MenuCommandState::Help].endpoint(command_help::command))
                    .branch(
                        dptree::case![MenuCommandState::Invite].endpoint(command_invite::command),
                    ),
            ),
        )
    }
}
