use log::{debug, error};
use teloxide::prelude::*;
use teloxide::types::{ChatId, InlineKeyboardMarkup, MessageId};
use teloxide::Bot;

pub async fn send_message<M, K>(bot: &Bot, chat_id: ChatId, message: M, keyboard: K)
where
    M: Into<String>,
    K: Into<Option<InlineKeyboardMarkup>>,
{
    match keyboard.into() {
        None => {
            let result = bot.send_message(chat_id, message).await;
            match result {
                Ok(_) => debug!("message sent to {}", chat_id),
                Err(error) => error!("{}", error),
            }
        }
        Some(keyboard) => {
            let result = bot.send_message(chat_id, message).reply_markup(keyboard).await;
            match result {
                Ok(_) => debug!("message with keyboard sent to {}", chat_id),
                Err(error) => error!("{}", error),
            }
        }
    }
}

pub async fn modify_message<M, K>(bot: Bot, chat_id: ChatId, message_id: MessageId, message: M, keyboard: K)
where
    M: Into<String>,
    K: Into<Option<InlineKeyboardMarkup>>,
{
    match keyboard.into() {
        None => {
            let result = bot.edit_message_text(chat_id, message_id, message).await;
            match result {
                Ok(_) => debug!("message {} edited to {}", message_id, chat_id),
                Err(error) => error!("{}", error),
            }
        }
        Some(keyboard) => {
            let result = bot
                .edit_message_text(chat_id, message_id, message)
                .reply_markup(keyboard)
                .await;
            match result {
                Ok(_) => debug!("message {} edited to {}", message_id, chat_id),
                Err(error) => error!("{}", error),
            }
        }
    }
}

pub fn is_message_from_group(message: &Message) -> bool {
    message.chat.is_group()
}
