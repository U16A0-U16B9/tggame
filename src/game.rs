mod game_status;
pub mod player;
use crate::schema::games;
use log::error;
use teloxide::types::{ChatId};
use uuid::Uuid;
use game_status::GameStatus;
use diesel::prelude::*;
use crate::game;
use crate::services::database::establish_connection;

#[derive(Queryable, Insertable)]
pub struct Game {
    pub id: Uuid,
    pub chat_id: String,
    pub status: GameStatus,
}

pub fn get_active_game(chat: &ChatId) -> QueryResult<Game> {
    use crate::schema::games::dsl::*;
    let connection = &mut establish_connection();
    games
        .filter(chat_id.eq(&chat.0.to_string()))
        .first::<Game>(connection)
}

pub fn create_game(chat: &ChatId) -> QueryResult<Game> {
    let connection = &mut establish_connection();
    let game = Game {
        id: Uuid::new_v4(),
        chat_id: chat.0.to_string(),
        status: GameStatus::LookingForGroup,
    };

    diesel::insert_into(games::table)
        .values(&game)
        .get_result(connection)
}
