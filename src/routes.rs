use rocket::{get, post};
use rocket::response::content;
use rocket::State;

use juniper::RootNode;

use crate::db::PrimaryDb;
use crate::graphql::schema::{QueryRoot, MutationRoot, Context};

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/")]
pub fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

#[get("/graphql?<request>")]
pub fn get_graphql_handler(
    context: PrimaryDb,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &Context { connection: context })
}

#[post("/graphql", data = "<request>")]
pub fn post_graphql_handler(
    context: PrimaryDb,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &Context { connection: context })
}
