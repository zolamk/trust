pub mod context;
mod mutation;
mod query;

use crate::config::Config;
use juniper_rocket::{GraphQLRequest, GraphQLResponse};
use mutation::*;
use query::*;
use rocket::{http::Status, response::content, State};

use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool},
};

pub type Schema = juniper::RootNode<'static, Query, Mutation>;

pub fn create_schema() -> Schema {
    return Schema::new(Query {}, Mutation {});
}

#[get("/graphiql")]
pub fn graphiql(config: State<Config>) -> content::Html<String> {
    let graphql_endpoint_url = config.instance_url.clone() + "/graphql";
    return juniper_rocket::graphiql_source(&graphql_endpoint_url);
}

#[post("/graphql", data = "<request>")]
pub fn graphql(request: GraphQLRequest, connection_pool: State<Pool<ConnectionManager<PgConnection>>>, config: State<Config>, schema: State<Schema>) -> GraphQLResponse {
    let connection = match connection_pool.get() {
        Ok(connection) => connection,
        Err(_err) => {
            return GraphQLResponse::custom(
                Status::InternalServerError,
                json!({
                    "code": "unable_to_get_connection"
                }),
            )
        }
    };

    return request.execute(&schema, &context::Context { connection });
}
