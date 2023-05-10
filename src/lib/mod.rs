pub mod schema;
pub mod models;
use diesel::{
    pg::PgConnection,
    prelude::*
};
use std::env;

pub fn establish_connection() -> PgConnection {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&db_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", db_url))
}