use crate::services::bootstrap;
use crate::services::seeds::seed;
use std::env;
use std::error::Error;

mod app;
pub mod commands;
mod game;
pub mod schema;
mod services;
pub mod utility;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    bootstrap::start();
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        app::init().await;
    }

    match args.get(1).expect("Cannot retrieve param").as_str() {
        "seed" => seed(),
        _ => panic!("Invalid param {}", args.first().unwrap()),
    };

    Ok(())
}
