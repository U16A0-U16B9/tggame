use crate::game::player::get_player;
use crate::game::{create_game, get_active_game};
use crate::utility::bot::message::send_message;
use log::error;
use teloxide::prelude::Message;
use teloxide::utils::command::BotCommands;
use teloxide::Bot;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
pub enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "Create New Game")]
    NewGame,
}

pub async fn new_game(bot: Bot, msg: Message) {
    if !msg.chat.is_group() {
        return;
    }

    let user = msg.from().expect("User not found");
    let player = get_player(&user.id);
    if let Err(_) = player {
        send_message(
            bot,
            msg.chat.id,
            format!(
                "Game cannot be created\
            \nPlease register by sending\
            \n/register or /start to bot\
            \nprivately"
            ),
            None,
        )
        .await;
        return;
    }

    let chat = msg.chat.id;
    let game_result = get_active_game(&chat);
    if let Ok(_) = game_result {
        send_message(bot, msg.chat.id, format!("Game already exists"), None).await;
    } else {
        create_game(&chat).unwrap_or_else(|error| {
            error!("Error creating game: {}", error);
            panic!("Error creating game for {}", chat)
        });
        send_message(
            bot,
            msg.chat.id,
            format!("Game created\nPlease type /join to join the game"),
            None,
        )
        .await;
    }
}
