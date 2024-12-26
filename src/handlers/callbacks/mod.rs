mod handler;

pub trait CallbacksHandler<'a> {
    fn setup_callbacks(self) -> Self;
}
