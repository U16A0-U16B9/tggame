use crate::game::player::{create_player, get_player};
use crate::utility::bot::message::send_message;
use teloxide::prelude::Message;
use teloxide::utils::command::BotCommands;
use teloxide::Bot;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
pub enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "register for new accounts")]
    Register,
    #[command(description = "same as register")]
    Start,
}

pub async fn register(bot: Bot, msg: Message) {
    if !msg.chat.is_private() {
        return;
    }

    let user = msg.from().expect("User not found");
    let firstname = user.first_name.clone();
    let username = user.username.clone().unwrap_or(firstname);

    let player = get_player(&user.id);

    if let Ok(_) = player {
        send_message(bot, msg.chat.id, format!("Welcome back {}", username), None).await;
    } else {
        let new_player = create_player(&user.id, username.as_str());
        if let Ok(_) = new_player {
            send_message(bot, msg.chat.id, format!("Welcome {}", username), None).await;
        } else {
            panic!("User not found")
        }
    }
}
