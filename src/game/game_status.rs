#[derive(diesel_derive_enum::DbEnum, Debug, PartialEq)]
#[ExistingTypePath = "crate::schema::sql_types::Status"]
pub enum GameStatus {
    LookingForGroup,
    InProgress,
    Completed,
}
