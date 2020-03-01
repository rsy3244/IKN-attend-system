use actix_web::{web, HttpResponse, Result};

use super::person::*;

use super::db::DbPool;

pub async fn attend(
    pool: web::Data<DbPool>,
    info: web::Path<(i32,)>
) -> Result<HttpResponse> {
    let connection = pool.get().expect("couldn't get db connection from pool");
    let input: i32 = info.0;
    let result = web::block(move || super::db::update_user_status(input, State::Leave, &connection))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;
    if let Some(result) = result {
        Ok(HttpResponse::Ok().json(result))
    } else {
        let res = HttpResponse::NotFound()
            .body(format!("No user found with id: {}", input));
        Ok(res)
    }
}

pub async fn leave(
    pool: web::Data<DbPool>,
    info: web::Path<(i32,)>
) -> Result<HttpResponse> {
    let connection = pool.get().expect("couldn't get db connection from pool");
    let input: i32 = info.0;
    let result = web::block(move || super::db::update_user_status(input, State::Leave, &connection))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;
    if let Some(result) = result {
        Ok(HttpResponse::Ok().json(result))
    } else {
        let res = HttpResponse::NotFound()
            .body(format!("No user found with id: {}", input));
        Ok(res)
    }
}

pub async fn get_all(
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    let connection = pool.get().expect("couldn't get db connection from pool");
    let ret = web::block(move || super::db::list_users(&connection))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;
    Ok(HttpResponse::Ok().json(ret))
}

pub async fn get_student(
    pool: web::Data<DbPool>,
    info: web::Path<(i32,)>
) -> Result<HttpResponse> {
    let connection = pool.get().expect("couldn't get db connection from pool");
    let input: i32 = info.0;
    let person = web::block(move || super::db::find_user_by_id(input, &connection))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    if let Some(person) = person {
        Ok(HttpResponse::Ok().json(person))
    } else {
        let res = HttpResponse::NotFound()
            .body(format!("No user found with id: {}", input));
        Ok(res)
    }
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
