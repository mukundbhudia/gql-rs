use async_graphql::*;

use crate::state::State;

pub struct Query;

#[Object]
impl Query {
    /// Returns the sum of a and b
    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn increment(&self, context: &Context<'_>, a: i32) -> Result<i32, anyhow::Error> {
        let _state = context.data::<State>().expect("No State found in state");

        Ok(a)
    }
}

pub type Schema = async_graphql::Schema<Query, Mutation, EmptySubscription>;

pub fn build_schema(state: State) -> Schema {
    Schema::build(Query, Mutation, EmptySubscription)
        .data(state)
        .finish()
}

// /// Returns the SDL of the GraphQL schema as a string.
// pub fn schema_sdl() -> String {
//     Schema::build(Query, Mutation, EmptySubscription)
//         .finish()
//         .sdl()
// }
