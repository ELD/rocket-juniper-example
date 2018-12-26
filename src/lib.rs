#![feature(decl_macro, proc_macro_hygiene, custom_attribute)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate juniper;

pub mod db;
pub mod routes;
pub mod models;
pub mod graphql;
mod schema;