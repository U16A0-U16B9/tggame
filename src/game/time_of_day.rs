#[derive(diesel_derive_enum::DbEnum, Debug, PartialEq)]
#[ExistingTypePath = "crate::schema::sql_types::TimeOfDay"]
pub enum TimeOfDay {
    Dawn,
    Day,
    Dusk,
    Night,
}

impl TimeOfDay {
    pub fn advance(&self) -> TimeOfDay {
        match self {
            TimeOfDay::Dawn => TimeOfDay::Day,
            TimeOfDay::Day => TimeOfDay::Dusk,
            TimeOfDay::Dusk => TimeOfDay::Night,
            TimeOfDay::Night => TimeOfDay::Dawn,
        }
    }
}
