use gql_rs::{start, Config};

#[async_std::main]
async fn main() {
    dotenv::dotenv().ok();
    let config = Config::new().expect("Couldn't load config");

    println!("Starting server with config: \n{}", config);
    start(config).await.expect("Failed to start server");
}
