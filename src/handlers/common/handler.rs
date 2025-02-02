use teloxide::{
    dispatching::{DpHandlerDescription, UpdateFilterExt},
    dptree::{self, Handler},
    prelude::{DependencyMap, Message},
    types::{MediaKind, MessageKind, Update},
};

use crate::routes::messages::handle_image_video_message;

use super::CommonHandler;

impl<'a: 'static> CommonHandler<'a>
    for Handler<
        'a,
        DependencyMap,
        Result<(), Box<dyn std::error::Error + Send + Sync>>,
        DpHandlerDescription,
    >
{
    fn setup_all_messages(self) -> Self {
        self.branch(
            Update::filter_message().branch(
                dptree::entry()
                    .filter_async(|msg: Message| async move {
                        if let MessageKind::Common(common) = &msg.kind {
                            matches!(common.media_kind, MediaKind::Photo(_) | MediaKind::Video(_))
                        } else {
                            false
                        }
                    })
                    .endpoint(handle_image_video_message),
            ),
        )
    }
}
