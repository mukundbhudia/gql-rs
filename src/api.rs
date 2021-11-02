use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use tide::{http::mime, Body, Request, Response, StatusCode};

pub async fn start() -> tide::Result<()> {
    let port = 5050;
    let mut server = tide::Server::new();

    server.at("/").get(hello);

    server.listen(format!("0.0.0.0:{}", port)).await?;

    Ok(())
}

async fn hello(_req: Request<()>) -> tide::Result<Response> {
    Ok(Response::builder(tide::StatusCode::Ok).body("OK").build())
}
