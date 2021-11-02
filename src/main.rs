use gql_rs::start;

#[async_std::main]
async fn main() {
    println!("Started server");
    start().await.expect("Failed to start server");
}
