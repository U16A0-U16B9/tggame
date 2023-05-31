use crate::game::cheats::Cheats;
use crate::game::ingame_player::get_ingame_player_from_username;
use crate::game::Game;
use crate::utility::string::find_username;

pub async fn handle_kill(game: &Game, message: String) -> Option<Cheats> {
    let username = find_username(message.as_str());
    if let None = username {
        return None;
    }

    let ingame_player = get_ingame_player_from_username(&username.unwrap(), game);
    if let Err(_) = ingame_player {
        return None;
    }

    Some(Cheats::Kill(ingame_player.unwrap()))
}
