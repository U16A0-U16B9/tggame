// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "status"))]
    pub struct Status;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "time_of_day"))]
    pub struct TimeOfDay;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Status;
    use super::sql_types::TimeOfDay;

    games (id) {
        id -> Uuid,
        chat_id -> Varchar,
        status -> Status,
        time_of_day -> TimeOfDay,
    }
}

diesel::table! {
    ingame_players (id) {
        id -> Uuid,
        game_id -> Uuid,
        player_id -> Uuid,
        role_id -> Nullable<Uuid>,
        is_alive -> Bool,
    }
}

diesel::table! {
    players (id) {
        id -> Uuid,
        user_id -> Varchar,
        username -> Varchar,
    }
}

diesel::table! {
    roles (id) {
        id -> Uuid,
        name -> Varchar,
        description -> Varchar,
        win_condition -> Varchar,
    }
}

diesel::joinable!(ingame_players -> games (game_id));
diesel::joinable!(ingame_players -> players (player_id));
diesel::joinable!(ingame_players -> roles (role_id));

diesel::allow_tables_to_appear_in_same_query!(games, ingame_players, players, roles,);
