use crate::game::game_status::GameStatus;
use crate::game::ingame_player::{get_ingame_players, set_role_to_ingame_player, IngamePlayer};
use crate::game::player::get_player_by_id;
use crate::game::role::Roles;
use crate::game::set_game_to_in_progress;
use crate::utility::bot::message::send_message;
use crate::utility::game::validate_game;
use rand::prelude::SliceRandom;
use rand::thread_rng;
use teloxide::prelude::Message;
use teloxide::types::ChatId;
use teloxide::Bot;

pub async fn start_game(bot: Bot, msg: Message) {
    let game = validate_game(&bot, &msg, GameStatus::LookingForGroup, "Cannot start game").await;
    if let None = game {
        return;
    }
    let game = game.unwrap();
    let ingame_players = get_ingame_players(&game).expect("cannot retrieve ingame players");
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
    set_ingame_roles(&bot, ingame_players).await;
    set_game_to_in_progress(&game).expect("Cannot start game");
}

pub async fn set_ingame_roles(bot: &Bot, mut ingame_players: Vec<IngamePlayer>) {
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
