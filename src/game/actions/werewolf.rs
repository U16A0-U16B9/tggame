use crate::game::actions::kill::schedule_kill;
use crate::game::ingame_player::get_ingame_player;
use crate::game::player::{
    get_all_non_werewolf_players_from_game, get_all_players_from_game_by_role, get_player_by_id,
};
use crate::game::role::Roles;
use crate::game::time_of_day::TimeOfDay;
use crate::game::{get_game_by_id, Game};
use crate::utility::bot::keyboard::make_keyboard;
use crate::utility::bot::message::send_message;
use crate::utility::string::parse_delimiter;
use crate::utility::uuid::ToBase64;
use std::collections::HashMap;
use teloxide::prelude::ChatId;
use teloxide::Bot;
use uuid::Uuid;

pub async fn action_message(bot: &Bot, game: &Game) {
    let werewolves = get_all_players_from_game_by_role(game, Roles::Werewolf).expect("cannot retrieve werewolves");
    let other_players = get_all_non_werewolf_players_from_game(game).expect("cannot retrieve players");
    let message = "You wake in middle of night, and start converting into werewolf, \
                         \nyou feel incredible hunger, choose player to kill";

    let player_options = other_players
        .into_iter()
        .map(|player| {
            (player.username, {
                let game_id = Uuid::base64_serialize(game.id);
                let player_id = Uuid::base64_serialize(player.id);
                format!("{}|{}|{}", "action|werewolf", game_id, player_id)
            })
        })
        .collect::<HashMap<String, String>>();

    let keyboard = make_keyboard(player_options, None);

    for werewolf in werewolves.iter() {
        send_message(
            bot,
            ChatId(werewolf.user_id.parse::<i64>().unwrap()),
            message,
            keyboard.clone(),
        )
        .await
    }
}

pub async fn callback(callback_data: &str) {
    let (pred_delimiter, post_delimiter) = parse_delimiter(callback_data);
    let player_id = Uuid::base64_deserialize(post_delimiter).expect("Unknown id");
    let player = get_player_by_id(player_id).expect("Unknown player");
    let game_id = Uuid::base64_deserialize(pred_delimiter).expect("Unknown id");
    let game = get_game_by_id(game_id).expect("Unknown game");
    let ingame_player = get_ingame_player(&player, &game).expect("Unknown ingame player");

    schedule_kill(ingame_player, TimeOfDay::Dawn)
}
