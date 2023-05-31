use crate::game::get_game_by_id;
use crate::game::ingame_player::{kill_ingame_player, IngamePlayer};
use crate::game::player::get_player_by_id;
use crate::utility::bot::message::send_message;
use teloxide::types::ChatId;
use teloxide::Bot;

pub enum DeathType {
    Executed,
    SuperNatural,
    Natural,
}

pub async fn kill_player(bot: &Bot, ingame_player: IngamePlayer, death_type: DeathType) {
    let player = get_player_by_id(ingame_player.player_id).expect("Unable to kill player");
    let game = get_game_by_id(ingame_player.game_id).expect("Unable to kill player");

    if !ingame_player.is_alive {
        send_message(
            bot,
            ChatId(game.chat_id.parse::<i64>().unwrap()),
            format!("{} is already dead", player.username),
            None,
        )
        .await;
        return;
    }

    kill_ingame_player(&ingame_player).expect("Unable to kill player");

    let message;
    match death_type {
        DeathType::Executed => {
            message = format!(
                "People decided that {} is for the blame\
            \n Executor walk up to the stage and swung his heavy axe and {} head rolled on ground",
                player.username, player.username
            )
        }
        DeathType::SuperNatural => {
            message = format!(
                "{} was found dead in his home\
            \n Body was mauled by some supernatural force",
                player.username
            )
        }
        DeathType::Natural => {
            message = format!(
                "{} was found dead in his home\
            \n It seems it was peaceful death",
                player.username
            )
        }
    }
    send_message(bot, ChatId(game.chat_id.parse::<i64>().unwrap()), message, None).await
}
