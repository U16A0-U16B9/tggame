use crate::game::ingame_player::get_alive_ingame_players_by_role;
use crate::game::role::Roles;
use crate::game::win_conditions::WinConditions::{Die, EliminateVillagers, EliminateWerewolves};
use crate::game::Game;
use strum_macros::{Display, EnumString};

#[derive(Debug, Display, PartialEq, EnumString)]
pub enum WinConditions {
    #[strum(serialize = "Eliminate all Werewolves")]
    EliminateWerewolves,
    #[strum(serialize = "You win if you die")]
    Die,
    #[strum(serialize = "Eliminate all Villagers")]
    EliminateVillagers,
}

pub fn handle_win_condition(game: &Game) -> Option<WinConditions> {
    let werewolf = get_alive_ingame_players_by_role(Roles::Werewolf, game).unwrap();
    if werewolf == 0 {
        return Some(EliminateWerewolves);
    }

    let tanner = get_alive_ingame_players_by_role(Roles::Tanner, game).unwrap();
    if tanner == 0 {
        return Some(Die);
    }

    let villager = get_alive_ingame_players_by_role(Roles::Villager, game).unwrap();
    let seer = get_alive_ingame_players_by_role(Roles::Seer, game).unwrap();

    if tanner + seer + villager == 0 {
        return Some(EliminateVillagers);
    }

    None
}
