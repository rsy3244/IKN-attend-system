use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use super::person::{NewPerson};

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connection to {}", database_url))
}

pub fn create_post<'a>(conn: &SqliteConnection, username: &'a str, role: &'a str) -> usize {
    use super::schema::people;

    let new_person = NewPerson {
        username,
        role,
    };

    diesel::insert_into(people::table)
        .values(&new_person)
        .execute(conn)
        .expect("Error")
}
        


