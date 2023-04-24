use std::env;
use crate::services::bootstrap;
use crate::services::seeds::seed;

mod app;
pub mod commands;
mod game;
pub mod schema;
mod services;

#[tokio::main]
async fn main() {
    bootstrap::start();
    let args: Vec<String> = env::args().collect();
    if args.is_empty() {
        app::init().await;
    }

    match args.get(1).expect("Cannot retrieve param").as_str() {
        "seed" => seed(),
        _ => panic!("Invalid param {}", args.first().unwrap())
    }
}
