mod game_status;
pub mod ingame_player;
pub mod player;
pub mod role;
pub mod win_conditions;
use crate::schema::games;
use crate::services::database::establish_connection;
use diesel::prelude::*;
use game_status::GameStatus;
use teloxide::types::ChatId;
use uuid::Uuid;

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
        .filter(status.ne(GameStatus::Completed))
        .first::<Game>(connection)
}

pub fn get_lfg_game(chat: &ChatId) -> QueryResult<Game> {
    use crate::schema::games::dsl::*;
    let connection = &mut establish_connection();
    games
        .filter(chat_id.eq(&chat.0.to_string()))
        .filter(status.eq(GameStatus::LookingForGroup))
        .first::<Game>(connection)
}

pub fn create_game(chat: &ChatId) -> QueryResult<Game> {
    let connection = &mut establish_connection();
    let game = Game {
        id: Uuid::new_v4(),
        chat_id: chat.0.to_string(),
        status: GameStatus::LookingForGroup,
    };

    diesel::insert_into(games::table).values(&game).get_result(connection)
}
