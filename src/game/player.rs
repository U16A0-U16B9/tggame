use crate::game::role::{Role, Roles};
use crate::game::win_conditions::WinConditions;
use crate::game::Game;
use crate::schema::players;
use crate::services::database::establish_connection;
use crate::utility::bot::message::send_message;
use crate::utility::bot::user::get_user_from_message;
use diesel::prelude::*;
use teloxide::types::{Message, UserId};
use teloxide::Bot;
use uuid::Uuid;

#[derive(Queryable, Insertable, Selectable)]
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

pub fn get_player_by_user_id(user: &UserId) -> QueryResult<Player> {
    use crate::schema::players::dsl::*;
    let connection = &mut establish_connection();
    players
        .filter(user_id.eq(&user.0.to_string()))
        .first::<Player>(connection)
}

pub fn get_player_by_id(player_id: Uuid) -> QueryResult<Player> {
    use crate::schema::players::dsl::*;
    let connection = &mut establish_connection();
    players.filter(id.eq(player_id)).first::<Player>(connection)
}

pub async fn get_player_from_message<T>(bot: &Bot, message: &Message, error_response: T) -> Option<Player>
where
    T: Into<Option<String>>,
{
    let user = get_user_from_message(&message);
    let player_result = get_player_by_user_id(&user.id);

    match player_result {
        Ok(player) => Some(player),
        Err(_) => {
            match error_response.into() {
                None => {
                    send_message(
                        bot,
                        message.chat.id,
                        format!(
                            "Please register by sending\
                            \n/register or /start to bot\
                            \nprivately"
                        ),
                        None,
                    )
                    .await
                }
                Some(error_message) => send_message(&bot, message.chat.id, error_message, None).await,
            }
            None
        }
    }
}

pub fn get_players_from_game_by_win_condition(game: &Game, win: &WinConditions) -> QueryResult<Vec<Player>> {
    use crate::schema::ingame_players::dsl::*;
    use crate::schema::players::dsl::*;
    use crate::schema::roles::dsl::*;
    let connection = &mut establish_connection();

    let role_model = roles
        .filter(win_condition.eq(win.to_string()))
        .load::<Role>(connection)
        .unwrap();

    let role_ids: Vec<Uuid> = role_model.iter().map(|role| role.id).collect();

    players
        .inner_join(ingame_players)
        .filter(game_id.eq(game.id))
        .filter(is_alive.eq(true))
        .filter(role_id.eq_any(role_ids))
        .select(Player::as_select())
        .load::<Player>(connection)
}

pub fn get_all_non_werewolf_players_from_game(game: &Game) -> QueryResult<Vec<Player>> {
    use crate::schema::ingame_players::dsl::*;
    use crate::schema::players::dsl::*;
    let connection = &mut establish_connection();

    let werewolf = Role::get(Roles::Werewolf).unwrap();
    players
        .inner_join(ingame_players)
        .filter(game_id.eq(game.id))
        .filter(is_alive.eq(true))
        .filter(role_id.ne(werewolf.id))
        .select(Player::as_select())
        .load::<Player>(connection)
}

pub fn get_all_non_seers_players_from_game(game: &Game) -> QueryResult<Vec<Player>> {
    use crate::schema::ingame_players::dsl::*;
    use crate::schema::players::dsl::*;
    let connection = &mut establish_connection();

    let werewolf = Role::get(Roles::Seer).unwrap();
    players
        .inner_join(ingame_players)
        .filter(game_id.eq(game.id))
        .filter(is_alive.eq(true))
        .filter(role_id.ne(werewolf.id))
        .select(Player::as_select())
        .load::<Player>(connection)
}

pub fn get_all_players_from_game_by_role(game: &Game, role: Roles) -> QueryResult<Vec<Player>> {
    use crate::schema::ingame_players::dsl::*;
    use crate::schema::players::dsl::*;
    let connection = &mut establish_connection();

    let role_model = Role::get(role).unwrap();
    players
        .inner_join(ingame_players)
        .filter(game_id.eq(game.id))
        .filter(is_alive.eq(true))
        .filter(role_id.eq(role_model.id))
        .select(Player::as_select())
        .load::<Player>(connection)
}
