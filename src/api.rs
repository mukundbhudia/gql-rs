use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use tide::{http::mime, Body, Request, Response, StatusCode};

use crate::{graphql::build_schema, state::State, Config, Schema};

pub async fn start(config: Config) -> tide::Result<()> {
    let port = config.port;
    let state = State::new(config)?;
    let config = state.config.clone();

    let schema = build_schema(state);
    let web_state = WebState { schema, config };

    let mut server = tide::Server::with_state(web_state);
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

async fn hello(_req: Request<WebState>) -> tide::Result<Response> {
    Ok(Response::builder(tide::StatusCode::Ok).body("OK").build())
}

async fn graphql_handler(req: Request<WebState>) -> tide::Result {
    use async_graphql_tide::{receive_request, respond};

    let schema = req.state().schema.clone();

    let gql_request = receive_request(req).await?;

    let response = schema.execute(gql_request).await;

    for error in &response.errors {
        tracing::error!("{:?}", log_graphql_error(error));
    }

    respond(response)
}

//  Tide application scope state.
#[derive(Clone)]
pub struct WebState {
    pub schema: Schema,
    pub config: Config,
}

#[derive(Debug)]
pub struct GraphQlError {
    message: String,
    path: Vec<String>,
}

fn log_graphql_error(error: &async_graphql::ServerError) -> GraphQlError {
    GraphQlError {
        message: (*error.message).to_string(),
        path: error
            .path
            .iter()
            .map(|segment| match segment {
                async_graphql::PathSegment::Field(field) => field.to_owned(),
                async_graphql::PathSegment::Index(index) => index.to_string(),
            })
            .collect::<Vec<String>>(),
    }
}
