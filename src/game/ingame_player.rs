use crate::game::player::Player;
use crate::game::role::{Role, Roles};
use crate::game::Game;
use crate::schema::ingame_players;
use crate::services::database::establish_connection;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Insertable, Associations, Selectable)]
#[diesel(belongs_to(Game))]
#[diesel(belongs_to(Player))]
#[diesel(belongs_to(Role))]
pub struct IngamePlayer {
    pub id: Uuid,
    pub game_id: Uuid,
    pub player_id: Uuid,
    pub role_id: Option<Uuid>,
    pub is_alive: bool,
}

pub fn create_ingame_player(player: &Player, game: &Game) -> QueryResult<IngamePlayer> {
    let connection = &mut establish_connection();

    let new_ingame_player = IngamePlayer {
        id: Uuid::new_v4(),
        game_id: game.id,
        player_id: player.id,
        role_id: None,
        is_alive: true,
    };

    diesel::insert_into(ingame_players::table)
        .values(&new_ingame_player)
        .get_result::<IngamePlayer>(connection)
}

pub fn get_ingame_player(player: &Player, game: &Game) -> QueryResult<IngamePlayer> {
    use crate::schema::ingame_players::dsl::*;
    let connection = &mut establish_connection();
    ingame_players
        .filter(player_id.eq(player.id))
        .filter(game_id.eq(game.id))
        .first::<IngamePlayer>(connection)
}

pub fn get_ingame_player_by_id(ingame_player_id: Uuid) -> QueryResult<IngamePlayer> {
    use crate::schema::ingame_players::dsl::*;
    let connection = &mut establish_connection();
    ingame_players
        .filter(id.eq(ingame_player_id))
        .first::<IngamePlayer>(connection)
}

pub fn get_ingame_player_from_username(player_username: &String, game: &Game) -> QueryResult<IngamePlayer> {
    use crate::schema::ingame_players::dsl::*;
    use crate::schema::players::dsl::*;
    let connection = &mut establish_connection();
    ingame_players
        .inner_join(players)
        .filter(username.eq(player_username))
        .filter(game_id.eq(game.id))
        .select(IngamePlayer::as_select())
        .first::<IngamePlayer>(connection)
}

pub fn get_ingame_players(game: &Game) -> QueryResult<Vec<IngamePlayer>> {
    use crate::schema::ingame_players::dsl::*;
    let connection = &mut establish_connection();
    ingame_players
        .filter(game_id.eq(game.id))
        .load::<IngamePlayer>(connection)
}

pub fn delete_ingame_player(ingame_player: IngamePlayer) -> QueryResult<usize> {
    use crate::schema::ingame_players::dsl::*;
    let connection = &mut establish_connection();
    diesel::delete(ingame_players.filter(id.eq(ingame_player.id))).execute(connection)
}

pub fn set_role_to_ingame_player(ingame_player: &IngamePlayer, role: Roles) -> QueryResult<IngamePlayer> {
    use crate::schema::ingame_players::dsl::*;
    use crate::schema::roles::dsl::*;
    let connection = &mut establish_connection();

    let role_model = roles
        .filter(name.eq(role.to_string()))
        .first::<Role>(connection)
        .unwrap();

    diesel::update(ingame_players.find(ingame_player.id))
        .set(role_id.eq(role_model.id))
        .get_result(connection)
}

pub fn kill_ingame_player(ingame_player: &IngamePlayer) -> QueryResult<IngamePlayer> {
    use crate::schema::ingame_players::dsl::*;
    let connection = &mut establish_connection();

    diesel::update(ingame_players.find(ingame_player.id))
        .set(is_alive.eq(false))
        .get_result(connection)
}

pub fn get_alive_ingame_players_by_role(role: Roles, game: &Game) -> QueryResult<i64> {
    use crate::schema::ingame_players::dsl::*;
    use crate::schema::roles::dsl::*;
    let connection = &mut establish_connection();

    let role_model = roles
        .filter(name.eq(role.to_string()))
        .first::<Role>(connection)
        .unwrap();

    ingame_players
        .filter(is_alive.eq(true))
        .filter(role_id.eq(role_model.id))
        .filter(game_id.eq(game.id))
        .count()
        .get_result::<i64>(connection)
}

pub fn get_dead_ingame_players_by_role(role: Roles, game: &Game) -> QueryResult<i64> {
    use crate::schema::ingame_players::dsl::*;
    use crate::schema::roles::dsl::*;
    let connection = &mut establish_connection();

    let role_model = roles
        .filter(name.eq(role.to_string()))
        .first::<Role>(connection)
        .unwrap();

    ingame_players
        .filter(is_alive.eq(false))
        .filter(role_id.eq(role_model.id))
        .filter(game_id.eq(game.id))
        .count()
        .get_result::<i64>(connection)
}
