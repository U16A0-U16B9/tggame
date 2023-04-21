// @generated automatically by Diesel CLI.

diesel::table! {
    players (id) {
        id -> Uuid,
        user_id -> Varchar,
        username -> Varchar,
    }
}
