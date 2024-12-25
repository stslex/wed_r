use teloxide::{
    dispatching::{DpHandlerDescription, HandlerExt, UpdateFilterExt},
    dptree::{self, Handler},
    prelude::DependencyMap,
    types::Update,
};

use crate::{
    commands::state::MenuCommandState,
    routes::{help, invite, start},
};

use super::MenuCommands;

impl<'a: 'static> MenuCommands<'a>
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
                    .branch(dptree::case![MenuCommandState::Start].endpoint(start))
                    .branch(dptree::case![MenuCommandState::Help].endpoint(help))
                    .branch(dptree::case![MenuCommandState::Invite].endpoint(invite)),
            ),
        )
    }
}
