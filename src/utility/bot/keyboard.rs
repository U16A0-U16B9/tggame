use std::collections::HashMap;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

pub fn make_keyboard<B>(menu: HashMap<String, String>, back: B) -> InlineKeyboardMarkup
where
    B: Into<Option<String>>,
{
    let mut keyboard = Vec::from_iter(
        menu.iter()
            .map(|(key, value)| vec![InlineKeyboardButton::callback(key.to_string(), value.to_string())]),
    );

    if let Some(value) = back.into() {
        keyboard.push(vec![InlineKeyboardButton::callback("← Back", value)])
    }

    InlineKeyboardMarkup::new(keyboard)
}
