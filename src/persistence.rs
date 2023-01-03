use diesel::prelude::*;
// use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use dotenv::dotenv;
use std::env;

// pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
