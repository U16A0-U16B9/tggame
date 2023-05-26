use crate::game::ingame_player::{get_ingame_player, get_ingame_players, set_role_to_ingame_player, IngamePlayer};
use crate::game::player::{get_player_by_id, get_player_from_message};
use crate::game::role::Roles;
use crate::game::{get_lfg_game, set_game_to_in_progress, Game};
use crate::utility::bot::message::{is_message_from_group, send_message};
use rand::prelude::SliceRandom;
use rand::thread_rng;
use teloxide::prelude::Message;
use teloxide::types::ChatId;
use teloxide::Bot;

pub async fn start_game(bot: Bot, msg: Message) {
    let game = validate_game(&bot, &msg).await;
    if let None = game {
        return;
    }
    let game = game.unwrap();
    set_ingame_roles(&bot, &msg, &game).await;
    set_game_to_in_progress(&game).expect("Cannot start game");
}

pub async fn validate_game(bot: &Bot, msg: &Message) -> Option<Game> {
    if !is_message_from_group(&msg) {
        return None;
    }
    let player = get_player_from_message(
        &bot,
        &msg,
        format!(
            "Cannot start the game\
        \nPlease register by sending\
        \n/register or /start to bot\
        \nprivately"
        ),
    )
    .await;
    if let None = player {
        return None;
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
        return None;
    }
    let game = game.unwrap();

    let ingame_player = get_ingame_player(&player, &game);
    if let Err(_) = ingame_player {
        send_message(
            &bot,
            msg.chat.id,
            format!(
                "Cannot start the game\
            \nYou are not in the game"
            ),
            None,
        )
        .await;
        return None;
    }

    return Some(game);
}
pub async fn set_ingame_roles(bot: &Bot, msg: &Message, game: &Game) {
    let mut ingame_players = get_ingame_players(&game).expect("cannot retrieve ingame players");

    // TODO: this should be increased
    if ingame_players.len() < 2 {
        send_message(
            &bot,
            msg.chat.id,
            format!(
                "Cannot start the game\
            \nNot enough players in the game"
            ),
            None,
        )
        .await;
        return;
    }
    ingame_players.shuffle(&mut thread_rng());

    let werewolf = ingame_players.pop().unwrap();
    let seer = ingame_players.pop().unwrap();

    set_role_to_ingame_player(&werewolf, Roles::Werewolf).expect("cannot asign role");
    notify_player_of_role(bot, &werewolf, Roles::Werewolf).await;
    set_role_to_ingame_player(&seer, Roles::Seer).expect("cannot asign role");
    notify_player_of_role(bot, &seer, Roles::Seer).await;

    for ingame_player in ingame_players.iter() {
        set_role_to_ingame_player(&ingame_player, Roles::Villager).expect("cannot asign role");
        notify_player_of_role(bot, &seer, Roles::Villager).await;
    }
}

async fn notify_player_of_role(bot: &Bot, ingame_player: &IngamePlayer, role: Roles) {
    let player = get_player_by_id(ingame_player.player_id).expect("cannot get player");
    match role {
        Roles::Villager => {
            send_message(
                bot,
                ChatId(player.user_id.parse::<i64>().unwrap()),
                "You are Villager",
                None,
            )
            .await
        }
        Roles::Tanner => {
            send_message(
                bot,
                ChatId(player.user_id.parse::<i64>().unwrap()),
                "You are Tanner",
                None,
            )
            .await
        }
        Roles::Seer => {
            send_message(
                bot,
                ChatId(player.user_id.parse::<i64>().unwrap()),
                "You are Seer",
                None,
            )
            .await
        }
        Roles::Werewolf => {
            send_message(
                bot,
                ChatId(player.user_id.parse::<i64>().unwrap()),
                "You are Werewolf",
                None,
            )
            .await
        }
    }
}
