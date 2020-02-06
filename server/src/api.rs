use actix_web::{web, HttpResponse, Result};

use super::person::*;
use super::diesel::prelude::*;

pub async fn attend(info: web::Path<(i32,)>) -> Result<HttpResponse> {
    use super::schema::people::dsl::*;
    let connection = super::db::establish_connection();
    let results = diesel::update(people)
        .filter(id.eq(info.0))
        .set(state.eq(1))
        .execute(&connection)
        .expect("Error loading people");

    get_student(info).await
}

pub async fn leave(info: web::Path<(i32,)>) -> Result<HttpResponse> {
    use super::schema::people::dsl::*;
    let connection = super::db::establish_connection();
    let results = diesel::update(people)
        .filter(id.eq(info.0))
        .set(state.eq(0))
        .execute(&connection)
        .expect("Error loading people");
    
    get_student(info).await
}

pub async fn get_all() -> Result<HttpResponse> {
    use super::schema::people::dsl::*;
    let connection = super::db::establish_connection();
    let results = people
        .load::<PersonRaw>(&connection)
        .expect("Error loading people");
    Ok(HttpResponse::Ok().json(
            results
    ))
}

pub async fn get_student(info: web::Path<(i32,)>) -> Result<HttpResponse> {
    use super::schema::people::dsl::*;
    let connection = super::db::establish_connection();
    let results = people
        .filter(id.eq(info.0))
        .load::<PersonRaw>(&connection)
        .expect("Error loading people");
    Ok(HttpResponse::Ok().json(
        results.iter().next()
    ))
}

//pub async fn signup<'a>(info: web::Json<NewPerson<'a>>) -> Result<HttpResponse> {
//    use super::schema::people;
//    let connection = super::db::establish_connection();
//    diesel::insert_into(people::table)
//        .values(&info)
//        .execute(connection)
//        .expect("Error signup");
//    Ok(HttpResponse::Ok())
//}


