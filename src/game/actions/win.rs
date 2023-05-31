use crate::game::player::get_players_from_game_by_win_condition;
use crate::game::win_conditions::WinConditions;
use crate::game::{set_game_to_to_completed, Game};
use crate::utility::bot::message::send_message;
use teloxide::prelude::ChatId;
use teloxide::Bot;

pub async fn set_win_condition(bot: &Bot, game: &Game, win: WinConditions) {
    let players = get_players_from_game_by_win_condition(game, &win).expect("Unable to set win condition");

    if players.is_empty() {
        return;
    }

    set_game_to_to_completed(&game).expect("Unable to set win condition");

    let message = match &win {
        WinConditions::EliminateWerewolves => format!("Werewolf are eliminated, Villagers win"),
        WinConditions::Die => format!("Tanner died, Tanner win"),
        WinConditions::EliminateVillagers => format!("Villagers are eliminated, Werewolf win"),
    };

    send_message(bot, ChatId(game.chat_id.parse::<i64>().unwrap()), message, None).await
}
