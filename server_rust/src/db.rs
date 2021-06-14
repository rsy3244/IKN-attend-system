use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use super::person::*;

pub type DbPool = diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<SqliteConnection>>;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connection to {}", database_url))
}

pub fn update_user_status(
    userid: i32,
    stat: State,
    conn: &SqliteConnection,
) -> Result<Option<Person>, diesel::result::Error> {
    use crate::schema::people::dsl::*;
    let stat = match stat {
        State::Leave => 0,
        State::Attend => 1,
        //_ => panic!("invalid state"),
    };

    let _result = diesel::update(people)
        .filter(id.eq(userid))
        .set(state.eq(stat))
        .execute(conn)?;
    //.expect("Error loading people");

    find_user_by_id(userid, conn)
}

pub fn list_users(conn: &SqliteConnection) -> Result<Vec<Person>, diesel::result::Error> {
    use crate::schema::people::dsl::*;
    people.load::<Person>(conn)
}

pub fn find_user_by_id(
    userid: i32,
    conn: &SqliteConnection,
) -> Result<Option<Person>, diesel::result::Error> {
    use crate::schema::people::dsl::*;
    let ret = people
        .filter(id.eq(userid))
        .first::<Person>(conn)
        .optional()?;

    Ok(ret)
}

pub fn register_user(
    conn: &SqliteConnection,
    new_person: &NewPerson,
) -> Result<usize, diesel::result::Error> {
    use super::schema::people;

    diesel::insert_into(people::table)
        .values(new_person)
        .execute(conn)
}
