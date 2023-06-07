use crate::game::actions::kill::kill_from_scheduled_action;
use crate::game::ingame_player::{get_ingame_players, IngamePlayer};
use crate::game::time_of_day::TimeOfDay;
use crate::game::Game;
use crate::schema::actions;
use crate::services::database::establish_connection;
use crate::utility::string::parse_delimiter;
use diesel::prelude::*;
use log::error;
use teloxide::prelude::CallbackQuery;
use teloxide::Bot;
use uuid::Uuid;

pub mod kill;
pub mod seer;
pub mod werewolf;
pub mod win;

#[derive(diesel_derive_enum::DbEnum, Debug, PartialEq)]
#[ExistingTypePath = "crate::schema::sql_types::Action"]
pub enum ActionTypes {
    Kill,
}

#[derive(Queryable, Insertable, Associations, Selectable)]
#[diesel(belongs_to(IngamePlayer))]
pub struct Action {
    pub id: Uuid,
    pub action: ActionTypes,
    pub ingame_player_id: Uuid,
    pub time_of_day: TimeOfDay,
    pub completed: bool,
}

pub fn schedule_action(action_type: ActionTypes, ingame_player: IngamePlayer, time: TimeOfDay) -> QueryResult<Action> {
    let connection = &mut establish_connection();
    let new_action = Action {
        id: Uuid::new_v4(),
        action: action_type,
        ingame_player_id: ingame_player.id,
        time_of_day: time,
        completed: false,
    };

    diesel::insert_into(actions::table)
        .values(&new_action)
        .get_result(connection)
}

pub fn set_action_to_completed(action_to_complete: &Action) -> QueryResult<Action> {
    use crate::schema::actions::dsl::*;
    let connection = &mut establish_connection();

    diesel::update(actions.find(action_to_complete.id))
        .set(completed.eq(true))
        .get_result(connection)
}

pub fn get_actions(ingame_players: Vec<IngamePlayer>, time: &TimeOfDay) -> QueryResult<Vec<Action>> {
    use crate::schema::actions::dsl::*;
    let connection = &mut establish_connection();

    let ingame_players: Vec<Uuid> = ingame_players.iter().map(|ingame_player| ingame_player.id).collect();

    actions
        .filter(completed.eq(false))
        .filter(time_of_day.eq(time))
        .filter(ingame_player_id.eq_any(ingame_players))
        .load::<Action>(connection)
}

pub async fn handle_actions(bot: &Bot, game: &Game, time: &TimeOfDay) {
    let ingame_players = get_ingame_players(&game).expect("cannot retrieve players");
    let actions = get_actions(ingame_players, time).expect("cannot retrieve actions");

    for action in actions.iter() {
        match action.action {
            ActionTypes::Kill => {
                kill_from_scheduled_action(bot, action).await;
            }
        }
        set_action_to_completed(action).expect("cannot complete action");
    }
}

pub async fn handle_action_callback(callback_data: &str, bot: Bot, q: CallbackQuery) {
    let (pred_delimiter, post_delimiter) = parse_delimiter(callback_data);
    match pred_delimiter {
        "" => {
            error!("no callback data found")
        }
        "werewolf" => werewolf::callback(post_delimiter).await,
        "seer" => seer::callback(post_delimiter, bot, q).await,
        &_ => {
            error!("no callback action found")
        }
    }
}
