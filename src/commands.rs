use crate::commands::direct_commands::register;
use crate::commands::group_commands::start_game::start_game;
use crate::commands::group_commands::{join_game, leave_game, new_game};
use crate::commands::tutorial_commands::main_tutorial_menu;
use crate::utility::bot::message::send_message;
use log::error;
use teloxide::prelude::*;
use teloxide::{utils::command::BotCommands, Bot};

pub mod direct_commands;
pub mod group_commands;
pub mod tutorial_commands;

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
    #[command(description = "How to play Game")]
    Tutorial,
    #[command(description = "Join Game")]
    Join,
    #[command(description = "Leave Game")]
    Leave,
    #[command(description = "Start Game")]
    StartGame,
}

pub async fn answer(bot: Bot, msg: Message, cmd: Command) {
    match cmd {
        Command::Help => parse_help(bot, msg).await,
        Command::Register => register(bot, msg).await,
        Command::Start => register(bot, msg).await,
        Command::NewGame => new_game(bot, msg).await,
        Command::Tutorial => main_tutorial_menu(bot, msg.chat.id, None).await,
        Command::Join => join_game(bot, msg).await,
        Command::Leave => leave_game(bot, msg).await,
        Command::StartGame => start_game(bot, msg).await,
    };
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
    send_message(&bot, msg.chat.id, message, None).await
}
