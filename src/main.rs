use gql_rs::{start, Config};
use tracing::info;

#[async_std::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();
    let config = Config::new().expect("Couldn't load config");

    info!("Starting server with config: \n{}", config);
    start(config).await.expect("Failed to start server");
}
