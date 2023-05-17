use crate::game::role::Role;
use crate::utility::bot::keyboard::make_keyboard;
use crate::utility::bot::message::modify_message;
use crate::utility::string::{parse_delimiter, truncate_last_delimiter};
use std::collections::HashMap;
use std::str::FromStr;
use teloxide::dispatching::dialogue::GetChatId;
use teloxide::prelude::*;
use teloxide::Bot;
use uuid::Uuid;

pub async fn handle_roles_callback(callback_data: &str, bot: Bot, q: CallbackQuery) {
    let (pred_delimiter, _) = parse_delimiter(callback_data);
    match pred_delimiter {
        "" => get_roles(bot, q).await,
        &_ => get_role(pred_delimiter, bot, q).await,
    }
}

async fn get_roles(bot: Bot, q: CallbackQuery) {
    let roles = Role::get_all();
    let name = "tutorial|roles".to_string();
    let roles_tutorial = roles
        .into_iter()
        .map(|role| (role.name.clone(), format!("{}|{}", name, role.id.to_string())))
        .collect::<HashMap<String, String>>();

    let back = truncate_last_delimiter(name.as_str());
    let keyboard = make_keyboard(roles_tutorial, back);
    modify_message(bot, q.chat_id().unwrap(), q.message.unwrap().id, name, keyboard).await;
}

async fn get_role(callback_data: &str, bot: Bot, q: CallbackQuery) {
    let id = Uuid::from_str(callback_data).expect("Unknown id");
    let role = Role::get_by_id(id).expect("Unknown role");
    let message = format!(
        "{}:\n{}\n\nWin Condition:\n{}",
        role.name, role.description, role.win_condition
    );
    let keyboard = make_keyboard(HashMap::new(), "tutorial|roles".to_string());
    modify_message(bot, q.chat_id().unwrap(), q.message.unwrap().id, message, keyboard).await;
}
