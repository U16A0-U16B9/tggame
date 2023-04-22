mod app;
pub mod commands;
mod game;
pub mod schema;
mod services;

#[tokio::main]
async fn main() {
    app::init().await;
}
