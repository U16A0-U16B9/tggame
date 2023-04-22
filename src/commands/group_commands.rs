use log::{debug, error};
use teloxide::Bot;
use teloxide::prelude::{Message, Requester};
use teloxide::utils::command::BotCommands;
use crate::game::{create_game, get_active_game};
use crate::game::player::get_player;

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
        bot.send_message(msg.chat.id, format!(
            "Game cannot be created\
            \nPlease register by sending\
            \n/register or /start to bot\
            \nprivately")).await;
        return;
    }

    let chat = msg.chat.id;
    let game_result = get_active_game(&chat);
    if let Ok(_) = game_result {
        bot.send_message(msg.chat.id, format!("Game already exists")).await;
    } else {
        let game = create_game(&chat).unwrap_or_else(|error| {
            error!("Error creating game: {}", error);
            panic!("Error creating game for {}", chat)
        });
        bot.send_message(msg.chat.id, format!("Game created\nPlease type /join to join the game")).await;
    }
}
