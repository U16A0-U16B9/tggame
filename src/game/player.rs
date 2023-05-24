use crate::schema::players;
use crate::services::database::establish_connection;
use diesel::prelude::*;
use teloxide::types::UserId;
use uuid::Uuid;

#[derive(Queryable, Insertable)]
pub struct Player {
    pub id: Uuid,
    pub user_id: String,
    pub username: String,
}

pub fn create_player(user_id: &UserId, username: &str) -> QueryResult<Player> {
    let connection = &mut establish_connection();
    let player = Player {
        id: Uuid::new_v4(),
        user_id: user_id.0.to_string(),
        username: username.to_string(),
    };

    diesel::insert_into(players::table)
        .values(&player)
        .get_result(connection)
}

pub fn get_player(user: &UserId) -> QueryResult<Player> {
    use crate::schema::players::dsl::*;
    let connection = &mut establish_connection();
    players
        .filter(user_id.eq(&user.0.to_string()))
        .first::<Player>(connection)
}
