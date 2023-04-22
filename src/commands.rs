use log::debug;
use teloxide::{Bot, utils::command::BotCommands};
use teloxide::prelude::*;
use crate::game::player::register;
use crate::game::new_game;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
pub enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "register for new accounts")]
    Register,
    #[command(description = "Create New Game")]
    NewGame,
}

pub async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
        Command::Register => {
            let user = msg.from().expect("User not found");
            let (player, message) = register(user).expect("User not found");
            debug!("{} player loaded", player.username);
            bot.send_message(msg.chat.id, message).await?
        },
        Command::NewGame => {
            let chat = msg.chat.id;
            let (game, message) = new_game(&chat).expect("Game not found");
            debug!("{} game loaded", game.chat_id);
            bot.send_message(msg.chat.id, message).await?
        }
    };

    Ok(())
}