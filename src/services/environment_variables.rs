use dotenvy::dotenv;

pub fn load() {
    dotenv().expect(".env file not found");
}
