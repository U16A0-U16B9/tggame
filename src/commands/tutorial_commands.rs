use crate::commands::tutorial_commands::roles::handle_roles_callback;
use crate::utility::bot::keyboard::make_keyboard;
use crate::utility::bot::message::{modify_message, send_message};
use crate::utility::string::parse_delimiter;
use log::error;
use std::collections::HashMap;
use teloxide::dispatching::dialogue::GetChatId;
use teloxide::prelude::*;
use teloxide::types::MessageId;
use teloxide::Bot;

mod roles;

pub async fn main_tutorial_menu<M>(bot: Bot, chat_id: ChatId, message_id: M)
where
    M: Into<Option<MessageId>>,
{
    let name = "tutorial".to_string();
    let mut menu = HashMap::new();
    menu.insert("Roles".to_string(), "tutorial|roles".to_string());
    let keyboard = make_keyboard(menu, None);

    match message_id.into() {
        None => send_message(&bot, chat_id, name, keyboard).await,
        Some(message_id) => modify_message(bot, chat_id, message_id, name, keyboard).await,
    }
}

pub async fn handle_tutorial_callback(callback_data: &str, bot: Bot, q: CallbackQuery) {
    let (pred_delimiter, post_delimiter) = parse_delimiter(callback_data);
    match pred_delimiter {
        "" => main_tutorial_menu(bot, q.chat_id().unwrap(), q.message.unwrap().id).await,
        "roles" => handle_roles_callback(post_delimiter, bot, q).await,
        &_ => {
            error!("no tutorial action found")
        }
    }
}
