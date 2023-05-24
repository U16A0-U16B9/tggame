use crate::game::player::Player;
use crate::game::role::Role;
use crate::game::Game;
use crate::schema::ingame_players;
use crate::services::database::establish_connection;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Insertable, Associations)]
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

pub fn delete_ingame_player(ingame_player: IngamePlayer) -> QueryResult<usize> {
    use crate::schema::ingame_players::dsl::*;
    let connection = &mut establish_connection();
    diesel::delete(ingame_players.filter(id.eq(ingame_player.id))).execute(connection)
}
