use crate::commands::direct_commands::register;
use crate::commands::group_commands::new_game;
use log::error;
use teloxide::prelude::*;
use teloxide::{utils::command::BotCommands, Bot};

pub mod direct_commands;
pub mod group_commands;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
pub enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "register for new accounts")]
    Register,
    #[command(description = "same as register")]
    Start,
    #[command(description = "Create New Game")]
    NewGame,
}

pub async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => parse_help(bot, msg).await,
        Command::Register => register(bot, msg).await,
        Command::Start => register(bot, msg).await,
        Command::NewGame => new_game(bot, msg).await,
    };

    Ok(())
}

pub async fn parse_help(bot: Bot, msg: Message) {
    let message: String;
    if msg.chat.is_private() {
        message = direct_commands::Command::descriptions().to_string();
    } else if msg.chat.is_group() {
        message = group_commands::Command::descriptions().to_string();
    } else {
        error!("Invalid chat : {}", msg.chat.id);
        panic!("Error: Invalid chat type");
    }
    bot.send_message(msg.chat.id, message).await.unwrap();
}
