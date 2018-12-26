#![feature(decl_macro, proc_macro_hygiene)]

use rocket::routes;

use libjuniperrocketexample::graphql::schema::{QueryRoot, MutationRoot};
use libjuniperrocketexample::routes::{self, Schema};
use libjuniperrocketexample::db::PrimaryDb;

fn main() {
    rocket::ignite()
        .attach(PrimaryDb::fairing())
        .manage(Schema::new(
            QueryRoot,
            MutationRoot,
        ))
        .mount("/", routes![
            routes::index,
            routes::get_graphql_handler,
            routes::post_graphql_handler
        ])
        .mount("/graphiql", routes![routes::graphiql])
        .launch();
}
