mod app;
mod game;
mod services;
pub mod commands;
pub mod schema;

#[tokio::main]
async fn main() {
    app::init().await;
}