mod handler;

pub trait CommonHandler<'a> {
    fn setup_all_messages(self) -> Self;
}
