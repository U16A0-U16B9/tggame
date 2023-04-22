use crate::game::player::{create_player, get_player};
use teloxide::prelude::{Message, Requester};
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
        bot.send_message(msg.chat.id, format!("Welcome back {}", username))
            .await
            .unwrap();
    } else {
        let new_player = create_player(&user.id, username.as_str());
        if let Ok(_) = new_player {
            bot.send_message(msg.chat.id, format!("Welcome {}", username))
                .await
                .unwrap();
        } else {
            panic!("User not found")
        }
    }
}
