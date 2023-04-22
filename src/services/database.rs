use diesel::pg::PgConnection;
use diesel::prelude::*;
use log::error;
use std::env;

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).unwrap_or_else(|error| {
        error!("Error: {}", error);
        panic!("Error connecting to {}", database_url)
    })
}
