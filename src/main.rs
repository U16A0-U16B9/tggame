mod app;
mod game;
mod services;

#[tokio::main]
async fn main() {
    app::init().await;
}
