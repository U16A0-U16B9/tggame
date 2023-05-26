use crate::game::ingame_player::{create_ingame_player, delete_ingame_player, get_ingame_player};
use crate::game::player::get_player_from_message;
use crate::game::{create_game, get_active_game, get_lfg_game};
use crate::utility::bot::message::{is_message_from_group, send_message};
use log::error;
use teloxide::prelude::Message;
use teloxide::utils::command::BotCommands;
use teloxide::Bot;

pub mod start_game;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
pub enum Command {
    #[command(description = "display this text.")]
    Help,
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

pub async fn new_game(bot: Bot, msg: Message) {
    if !is_message_from_group(&msg) {
        return;
    }

    let player = get_player_from_message(
        &bot,
        &msg,
        format!(
            "Game cannot be created\
        \nPlease register by sending\
        \n/register or /start to bot\
        \nprivately"
        ),
    )
    .await;
    if let None = player {
        return;
    }

    let chat = msg.chat.id;
    let game_result = get_active_game(&chat);
    if let Ok(_) = game_result {
        send_message(&bot, msg.chat.id, format!("Game already exists"), None).await;
    } else {
        create_game(&chat).unwrap_or_else(|error| {
            error!("Error creating game: {}", error);
            panic!("Error creating game for {}", chat)
        });
        send_message(
            &bot,
            msg.chat.id,
            format!("Game created\nPlease type /join to join the game"),
            None,
        )
        .await;
    }
}

pub async fn join_game(bot: Bot, msg: Message) {
    if !msg.chat.is_group() {
        return;
    }

    let player = get_player_from_message(
        &bot,
        &msg,
        format!(
            "Cannot join the game\
            \nPlease register by sending\
            \n/register or /start to bot\
            \nprivately"
        ),
    )
    .await;
    if let None = player {
        return;
    }
    let player = player.unwrap();

    let game = get_lfg_game(&msg.chat.id);
    if let Err(_) = game {
        send_message(
            &bot,
            msg.chat.id,
            format!(
                "Cannot join the game\
            \ngame not found"
            ),
            None,
        )
        .await;
        return;
    }
    let game = game.unwrap();

    let ingame_player = get_ingame_player(&player, &game);
    if let Ok(_) = ingame_player {
        send_message(&bot, msg.chat.id, format!("You are already in the game"), None).await;
        return;
    }
    let ingame_player = create_ingame_player(&player, &game);
    match ingame_player {
        Ok(_) => {
            send_message(
                &bot,
                msg.chat.id,
                format!("Player {} joined the game", player.username),
                None,
            )
            .await;
        }
        Err(error) => {
            error!("{}", error);
            send_message(&bot, msg.chat.id, format!("Cannot join game"), None).await;
        }
    }
}

pub async fn leave_game(bot: Bot, msg: Message) {
    if !msg.chat.is_group() {
        return;
    }

    let player = get_player_from_message(
        &bot,
        &msg,
        format!(
            "Cannot leave the game\
            \nPlease register by sending\
            \n/register or /start to bot\
            \nprivately"
        ),
    )
    .await;
    if let None = player {
        return;
    }
    let player = player.unwrap();

    let game = get_lfg_game(&msg.chat.id);
    if let Err(_) = game {
        send_message(
            &bot,
            msg.chat.id,
            format!(
                "Cannot leave game\
            \ngame not found"
            ),
            None,
        )
        .await;
        return;
    }
    let game = game.unwrap();

    let ingame_player = get_ingame_player(&player, &game);
    if let Err(_) = ingame_player {
        send_message(
            &bot,
            msg.chat.id,
            format!(
                "Cannot leave game\
            \nYou are not in the game"
            ),
            None,
        )
        .await;
        return;
    }

    delete_ingame_player(ingame_player.unwrap()).expect("cannot delete ingame player");

    send_message(
        &bot,
        msg.chat.id,
        format!("Player {} left the game", player.username),
        None,
    )
    .await;
}
