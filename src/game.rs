mod game_status;
mod player;
use teloxide::types::{ChatId};
use uuid::Uuid;
use game_status::GameStatus;
use player::Player;

struct Game {
    game_id: Uuid,
    chat_id: ChatId,
    status: GameStatus,
    players: Vec<Player>
}