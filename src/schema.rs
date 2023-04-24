// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "status"))]
    pub struct Status;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Status;

    games (id) {
        id -> Uuid,
        chat_id -> Varchar,
        status -> Status,
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

diesel::allow_tables_to_appear_in_same_query!(games, players, roles,);
