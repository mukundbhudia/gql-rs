use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use tide::{http::mime, Body, Request, Response, StatusCode};

pub async fn start() -> tide::Result<()> {
    let port = 5050;
    let mut server = tide::Server::new();

    server.at("/").get(hello);
    server.at("/query").post(graphql_handler);
    server.at("/graphiql").get(|_| async move {
        let mut res = Response::new(StatusCode::Ok);
        res.set_body(Body::from_string(playground_source(
            GraphQLPlaygroundConfig::new("/query"),
        )));
        res.set_content_type(mime::HTML);
        Ok(res)
    });

    server.listen(format!("0.0.0.0:{}", port)).await?;

    Ok(())
}

async fn hello(_req: Request<()>) -> tide::Result<Response> {
    Ok(Response::builder(tide::StatusCode::Ok).body("OK").build())
}

async fn graphql_handler(req: Request<()>) -> tide::Result {
    use async_graphql::*;
    use async_graphql_tide::{receive_request, respond};

    struct Query;

    #[Object]
    impl Query {
        async fn add(&self, a: i32, b: i32) -> i32 {
            a + b
        }
    }

    let gql_request = receive_request(req).await?;
    let schema = Schema::new(Query, EmptyMutation, EmptySubscription);
    let response = schema.execute(gql_request).await;

    respond(response)
}
