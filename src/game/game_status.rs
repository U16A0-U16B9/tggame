#[derive(diesel_derive_enum::DbEnum, Debug)]
#[ExistingTypePath = "crate::schema::sql_types::Status"]
pub enum GameStatus {
    LookingForGroup,
    InProgress,
    Completed,
}
