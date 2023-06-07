use crate::game::ingame_player::get_ingame_player;
use crate::game::player::{get_all_non_seers_players_from_game, get_all_players_from_game_by_role, get_player_by_id};
use crate::game::role::{Role, Roles};
use crate::game::{get_game_by_id, Game};
use crate::utility::bot::keyboard::make_keyboard;
use crate::utility::bot::message::send_message;
use crate::utility::string::parse_delimiter;
use crate::utility::uuid::ToBase64;
use std::collections::HashMap;
use teloxide::dispatching::dialogue::GetChatId;
use teloxide::prelude::{CallbackQuery, ChatId};
use teloxide::Bot;
use uuid::Uuid;

pub async fn action_message(bot: &Bot, game: &Game) {
    let seers = get_all_players_from_game_by_role(game, Roles::Seer).expect("cannot retrieve seers");
    let other_players = get_all_non_seers_players_from_game(game).expect("cannot retrieve players");
    let message = "You wake in middle of night, As a seer you have duty to find werewolves\
                         \nchoose player to follow";

    let player_options = other_players
        .into_iter()
        .map(|player| {
            (player.username, {
                let game_id = Uuid::base64_serialize(game.id);
                let player_id = Uuid::base64_serialize(player.id);
                format!("{}|{}|{}", "action|seer", game_id, player_id)
            })
        })
        .collect::<HashMap<String, String>>();

    let keyboard = make_keyboard(player_options, None);

    for seer in seers.iter() {
        send_message(
            bot,
            ChatId(seer.user_id.parse::<i64>().unwrap()),
            message,
            keyboard.clone(),
        )
        .await
    }
}

pub async fn callback(callback_data: &str, bot: Bot, q: CallbackQuery) {
    let (pred_delimiter, post_delimiter) = parse_delimiter(callback_data);
    let player_id = Uuid::base64_deserialize(post_delimiter).expect("Unknown id");
    let player = get_player_by_id(player_id).expect("Unknown player");
    let game_id = Uuid::base64_deserialize(pred_delimiter).expect("Unknown id");
    let game = get_game_by_id(game_id).expect("Unknown game");
    let ingame_player = get_ingame_player(&player, &game).expect("Unknown ingame player");
    let role = Role::get_by_id(ingame_player.role_id.unwrap()).expect("Unknown role");

    if role.name.eq(&Roles::Werewolf.to_string()) {
        send_message(
            &bot,
            q.chat_id().unwrap(),
            format!("{} is a werewolf", player.username),
            None,
        )
        .await;
    } else {
        send_message(
            &bot,
            q.chat_id().unwrap(),
            format!("{} is not a werewolf", player.username),
            None,
        )
        .await;
    }
}
