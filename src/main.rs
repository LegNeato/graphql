#[macro_use]
extern crate juniper;

mod database;
mod schema;

use juniper::EmptySubscription;
use std::sync::Arc;
use warp::Filter;
use crate::schema::{Context, QueryRoot, MutationRoot, Schema};

#[tokio::main]
async fn main() {
    let client = database::establish_connection().await;
    let client = Arc::new(client);

    let schema = Schema::new(
        QueryRoot,
        MutationRoot,
        EmptySubscription::<Context>::new(),
    );

    let state = warp::any().map(move || Context::with(&client));

    let graphiql_filter = warp::path("graphiql")
        .and(juniper_warp::graphiql_filter("/graphql", None));

    let graphql_filter = warp::path("graphql")
        .and(juniper_warp::make_graphql_filter(schema, state.boxed()));

    warp::serve(graphql_filter.or(graphiql_filter))
        .run(([127, 0, 0, 1], 8080))
        .await
}
