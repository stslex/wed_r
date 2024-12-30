use teloxide::types::KeyboardButton;

mod keyboard_buttons;

pub trait KeyboardButtonUtil {
    fn create_keyboard_buttons(&self) -> Vec<Vec<KeyboardButton>>;
}

pub trait ResultUtil<T, E>
where
    T: Clone,
{
    async fn to_result<'a, F>(&'a self, mapper: F) -> Result<T, E>
    where
        F: FnOnce() -> E + 'a;
}

impl<T, E> ResultUtil<T, E> for Option<T>
where
    T: Clone,
{
    async fn to_result<'a, F>(&'a self, mapper: F) -> Result<T, E>
    where
        F: FnOnce() -> E + 'a,
    {
        match self {
            Some(value) => Ok(value.to_owned()),
            None => Err(mapper()),
        }
    }
}
