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
