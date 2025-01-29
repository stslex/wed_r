use teloxide::{
    dispatching::{dialogue::InMemStorage, DpHandlerDescription, HandlerExt, UpdateFilterExt},
    dptree::{self, Handler},
    prelude::{DependencyMap, Message},
    types::Update,
};

use crate::{
    config::CreateUserState,
    handlers::state::MenuAdminCommandState,
    routes::{
        admin::{
            create_user::{command_create_user, handle_create_user_state},
            get_all_created_users::command_get_all_created_users,
        },
        command_open_main_menu,
        start::command_start,
    },
};

use super::AdminMenuCommandsHandler;

impl<'a: 'static> AdminMenuCommandsHandler<'a>
    for Handler<
        'a,
        DependencyMap,
        Result<(), Box<dyn std::error::Error + Send + Sync>>,
        DpHandlerDescription,
    >
{
    fn setup_admin_menu_commands(self) -> Self {
        self.branch(
            Update::filter_message()
                .enter_dialogue::<Message, InMemStorage<CreateUserState>, CreateUserState>()
                .branch(
                    dptree::entry()
                        .filter_command::<MenuAdminCommandState>()
                        .branch(dptree::case![MenuAdminCommandState::Start].endpoint(command_start))
                        .branch(
                            dptree::case![MenuAdminCommandState::UserMenu]
                                .endpoint(command_open_main_menu::command),
                        )
                        .branch(
                            dptree::case![MenuAdminCommandState::CreatedUsers]
                                .endpoint(command_get_all_created_users),
                        ),
                )
                .branch(
                    dptree::entry()
                        .enter_dialogue::<Message, InMemStorage<CreateUserState>, CreateUserState>()
                        .branch(
                            dptree::entry()
                                .filter_command::<MenuAdminCommandState>()
                                .branch(
                                    dptree::case![MenuAdminCommandState::CreateUser]
                                        .endpoint(command_create_user),
                                ),
                        )
                        .branch(dptree::endpoint(handle_create_user_state)),
                ),
        )
    }
}
