use teloxide::types::UserId;
use uuid::Uuid;

pub struct Player {
    player_id: Uuid,
    user_id: UserId
}