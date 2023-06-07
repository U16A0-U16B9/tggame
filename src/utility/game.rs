use crate::game::actions::win::set_win_condition;
use crate::game::actions::{handle_actions, seer, werewolf};
use crate::game::game_status::GameStatus;
use crate::game::ingame_player::get_ingame_player;
use crate::game::player::get_player_from_message;
use crate::game::time_of_day::TimeOfDay;
use crate::game::win_conditions::handle_win_condition;
use crate::game::{get_active_game, get_lfg_game, update_game_time_of_day, Game};
use crate::utility::bot::message::{is_message_from_group, send_message};
use teloxide::prelude::{ChatId, Message};
use teloxide::Bot;

pub async fn validate_game<T>(bot: &Bot, msg: &Message, game_status: GameStatus, error_msg: T) -> Option<Game>
where
    T: Into<String>,
{
    if !is_message_from_group(&msg) {
        return None;
    }

    let error_msg = error_msg.into();
    let player = get_player_from_message(
        &bot,
        &msg,
        format!(
            "{}\
            \nPlease register by sending\
            \n/register or /start to bot\
            \nprivately",
            &error_msg
        ),
    )
    .await;
    if let None = player {
        return None;
    }
    let player = player.unwrap();

    let game = match game_status {
        GameStatus::LookingForGroup => get_lfg_game(&msg.chat.id),
        GameStatus::InProgress => get_active_game(&msg.chat.id),
        GameStatus::Completed => todo!(),
    };

    if let Err(_) = game {
        send_message(
            &bot,
            msg.chat.id,
            format!(
                "{}\
                \ngame not found",
                &error_msg
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
                "{}\
                \nYou are not in the game",
                &error_msg
            ),
            None,
        )
        .await;
        return None;
    }

    return Some(game);
}

pub async fn advance_game_time_of_day(bot: &Bot, game: &Game) {
    let new_time = game.time_of_day.advance();
    handle_actions(bot, game, &new_time).await;
    let win = handle_win_condition(game);

    if let Some(win_condition) = win {
        return set_win_condition(bot, game, win_condition).await;
    }

    let message = match new_time {
        TimeOfDay::Dawn => {
            format!("Sun is rising, people are waking up, nothing unusual happened during night")
        }
        TimeOfDay::Day => {
            format!("day passes by")
        }
        TimeOfDay::Dusk => {
            format!("Sun is setting, folks are getting ready to sleep")
        }
        TimeOfDay::Night => {
            werewolf::action_message(bot, game).await;
            seer::action_message(bot, game).await;
            format!("Night is dark, almost everybody is asleep")
        }
    };

    update_game_time_of_day(game, new_time).expect("cannot update game time of day");

    send_message(bot, ChatId(game.chat_id.parse::<i64>().unwrap()), message, None).await
}
