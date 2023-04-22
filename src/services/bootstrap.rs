use crate::services::database;
use crate::services::environment_variables;
use log::info;

pub fn start() {
    pretty_env_logger::init();
    info!("🥾 Bootstrap started...");
    info!("⚙️ Upsetting environment:");
    environment_variables::load();
    info!("⚙️ Environment set");
    info!("⚙️ Connecting to database:");
    database::establish_connection();
    info!("⚙️ Database connection established");
}
