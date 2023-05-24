use crate::game::ingame_player::{create_ingame_player, delete_ingame_player, get_ingame_player};
use crate::game::player::get_player;
use crate::game::{create_game, get_active_game, get_lfg_game};
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
    #[command(description = "How to play Game")]
    Tutorial,
    #[command(description = "Join Game")]
    Join,
    #[command(description = "Leave Game")]
    Leave,
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

pub async fn join_game(bot: Bot, msg: Message) {
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
                "Cannot join game\
            \nPlease register by sending\
            \n/register or /start to bot\
            \nprivately"
            ),
            None,
        )
        .await;
        return;
    }

    let game = get_lfg_game(&msg.chat.id);
    if let Err(_) = game {
        send_message(
            bot,
            msg.chat.id,
            format!(
                "Cannot join game\
            \ngame not found"
            ),
            None,
        )
        .await;
        return;
    }

    let player = player.unwrap();
    let game = game.unwrap();

    let ingame_player = get_ingame_player(&player, &game);
    if let Ok(_) = ingame_player {
        send_message(bot, msg.chat.id, format!("You already joined game"), None).await;
        return;
    }
    let ingame_player = create_ingame_player(&player, &game);
    match ingame_player {
        Ok(_) => {
            send_message(
                bot,
                msg.chat.id,
                format!("Player {} joined the game", player.username),
                None,
            )
            .await;
        }
        Err(error) => {
            error!("{}", error);
            send_message(bot, msg.chat.id, format!("Cannot join game"), None).await;
        }
    }
}

pub async fn leave_game(bot: Bot, msg: Message) {
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
                "Cannot leave game\
            \nPlease register by sending\
            \n/register or /start to bot\
            \nprivately"
            ),
            None,
        )
        .await;
        return;
    }

    let game = get_lfg_game(&msg.chat.id);
    if let Err(_) = game {
        send_message(
            bot,
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

    let player = player.unwrap();
    let game = game.unwrap();

    let ingame_player = get_ingame_player(&player, &game);
    if let Err(_) = ingame_player {
        send_message(
            bot,
            msg.chat.id,
            format!(
                "Cannot leave game\
            \nYou are not in game"
            ),
            None,
        )
        .await;
        return;
    }

    delete_ingame_player(ingame_player.unwrap()).expect("cannot delete ingame player");

    send_message(
        bot,
        msg.chat.id,
        format!("Player {} left the game", player.username),
        None,
    )
    .await;
}
