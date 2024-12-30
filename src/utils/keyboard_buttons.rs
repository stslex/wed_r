use teloxide::types::{BotCommand, KeyboardButton};

use super::KeyboardButtonUtil;

impl KeyboardButtonUtil for Vec<BotCommand> {
    fn create_keyboard_buttons(&self) -> Vec<Vec<KeyboardButton>> {
        self.iter()
            .map(|c| KeyboardButton::new(c.command.clone()))
            .collect::<Vec<_>>()
            .chunks(2)
            .map(|chunk| chunk.to_vec())
            .collect::<Vec<_>>()
    }
}
