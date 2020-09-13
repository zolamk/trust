pub mod context;
mod mutation;
mod query;

use crate::{
    config::Config,
    crypto::{jwt::JWT, Error as CryptoError},
    handlers::Error as HandlerError,
    mailer::EmailTemplates,
    operator_signature::{Error as OperatorSignatureError, OperatorSignature},
};
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
    let source = format!(
        r#"<html>
        <head>
            <title>GraphiQL</title>
            <script>
            window.__env = {{
                graphqlEndpoint: '{}/graphql',
                headers: '',
                variables: '',
                query: ''
            }};
            </script>
        </head>
        <body>
            <div id='loading'></div>
            <div id='content' class='mainContent'></div>
            <link rel='stylesheet' href='https://cdnjs.cloudflare.com/ajax/libs/font-awesome/4.7.0/css/font-awesome.min.css'/>
            <link rel='stylesheet' href='https://graphiql-online.com/dist/main.css' charset='UTF-8'/>
            <script src='https://graphiql-online.com/dist/vendor.js' charset='UTF-8'></script>
            <script src='https://graphiql-online.com/dist/main.js' charset='UTF-8'></script>

        </body>
        </html>
    "#,
        config.instance_url
    );

    return content::Html::<String>(source);
}

#[post("/graphql", data = "<request>")]
pub fn graphql(
    request: GraphQLRequest,
    connection_pool: State<Pool<ConnectionManager<PgConnection>>>,
    config: State<Config>,
    schema: State<Schema>,
    operator_signature: Result<OperatorSignature, OperatorSignatureError>,
    email_templates: State<EmailTemplates>,
    token: Result<JWT, CryptoError>,
) -> GraphQLResponse {
    if operator_signature.is_err() {
        let err = HandlerError::from(operator_signature.err().unwrap());
        return GraphQLResponse::custom(Status::new(err.code, "error"), err.body);
    }

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

    return request.execute(
        &schema,
        &context::Context {
            connection,
            config: config.inner().clone(),
            email_templates: email_templates.inner().clone(),
            operator_signature: operator_signature.unwrap(),
            token,
        },
    );
}
