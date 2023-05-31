pub mod actions;
pub mod cheats;
pub mod game_status;
pub mod ingame_player;
pub mod player;
pub mod role;
pub mod time_of_day;
pub mod win_conditions;
use crate::game::time_of_day::TimeOfDay;
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
    pub time_of_day: TimeOfDay,
}

pub fn get_active_game(chat: &ChatId) -> QueryResult<Game> {
    use crate::schema::games::dsl::*;
    let connection = &mut establish_connection();
    games
        .filter(chat_id.eq(&chat.0.to_string()))
        .filter(status.eq(GameStatus::InProgress))
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
        time_of_day: TimeOfDay::Dusk,
    };

    diesel::insert_into(games::table).values(&game).get_result(connection)
}

pub fn set_game_to_in_progress(game: &Game) -> QueryResult<Game> {
    use crate::schema::games::dsl::*;
    let connection = &mut establish_connection();

    diesel::update(games.find(game.id))
        .set(status.eq(GameStatus::InProgress))
        .get_result::<Game>(connection)
}

pub fn set_game_to_to_completed(game: &Game) -> QueryResult<Game> {
    use crate::schema::games::dsl::*;
    let connection = &mut establish_connection();

    diesel::update(games.find(game.id))
        .set(status.eq(GameStatus::Completed))
        .get_result::<Game>(connection)
}

pub fn get_game_by_id(game_id: Uuid) -> QueryResult<Game> {
    use crate::schema::games::dsl::*;
    let connection = &mut establish_connection();
    games.filter(id.eq(game_id)).first::<Game>(connection)
}

pub fn update_game_time_of_day(game: &Game, time: TimeOfDay) -> QueryResult<Game> {
    use crate::schema::games::dsl::*;
    let connection = &mut establish_connection();

    diesel::update(games.find(game.id))
        .set(time_of_day.eq(time))
        .get_result::<Game>(connection)
}
