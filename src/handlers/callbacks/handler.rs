use teloxide::{
    dispatching::{DpHandlerDescription, UpdateFilterExt},
    dptree::Handler,
    prelude::DependencyMap,
    types::Update,
};

use crate::routes::callback_invite_query;

use super::CallbacksHandler;

impl<'a: 'static> CallbacksHandler<'a>
    for Handler<
        'a,
        DependencyMap,
        Result<(), Box<dyn std::error::Error + Send + Sync>>,
        DpHandlerDescription,
    >
{
    fn setup_callbacks(self) -> Self {
        self.branch(Update::filter_callback_query().endpoint(callback_invite_query::callback))
    }
}
