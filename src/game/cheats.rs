use crate::game::actions::kill::{kill_player, DeathType};
use crate::game::actions::win::set_win_condition;
use crate::game::cheats::kill::handle_kill;
use crate::game::game_status::GameStatus;
use crate::game::ingame_player::IngamePlayer;
use crate::game::role::Roles;
use crate::game::win_conditions::WinConditions;
use crate::utility::game::{advance_game_time_of_day, validate_game};
use teloxide::prelude::Message;
use teloxide::Bot;

pub mod kill;

pub enum Cheats {
    WinInstantly(Roles),
    Kill(IngamePlayer),
    AdvanceTime,
}

pub async fn handle_cheats(bot: Bot, msg: Message) {
    let game = validate_game(&bot, &msg, GameStatus::InProgress, "cannot use cheat").await;
    if let None = game {
        return;
    }
    let game = game.unwrap();

    let message = msg.text().unwrap_or("").to_string();
    let cheat = match &message {
        x if x.eq("omnomnom") => Some(Cheats::WinInstantly(Roles::Werewolf)),
        x if x.eq("time passes by") => Some(Cheats::AdvanceTime),
        x if x.starts_with("rip ") => handle_kill(&game, message).await,
        _ => None,
    };

    if let None = cheat {
        return;
    }

    match cheat.unwrap() {
        Cheats::WinInstantly(role) => match role {
            Roles::Villager => set_win_condition(&bot, &game, WinConditions::EliminateWerewolves).await,
            Roles::Tanner => set_win_condition(&bot, &game, WinConditions::Die).await,
            Roles::Seer => set_win_condition(&bot, &game, WinConditions::EliminateWerewolves).await,
            Roles::Werewolf => set_win_condition(&bot, &game, WinConditions::EliminateVillagers).await,
        },
        Cheats::Kill(player) => kill_player(&bot, player, DeathType::Natural).await,
        Cheats::AdvanceTime => advance_game_time_of_day(&bot, &game).await,
    }
}
