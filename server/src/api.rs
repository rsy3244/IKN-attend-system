use actix_web::{web, HttpResponse, Result};

use super::person::*;

use super::db::DbPool;

pub async fn attend(pool: web::Data<DbPool>, info: web::Path<(i32,)>) -> Result<HttpResponse> {
    let connection = pool.get().map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;
    let input: i32 = info.0;
    let result =
        web::block(move || super::db::update_user_status(input, State::Attend, &connection))
            .await
            .map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            })?;
    if let Some(result) = result {
        Ok(HttpResponse::Ok().json(result))
    } else {
        let res = HttpResponse::NotFound().body(format!("No user found with id: {}", input));
        Ok(res)
    }
}

pub async fn leave(pool: web::Data<DbPool>, info: web::Path<(i32,)>) -> Result<HttpResponse> {
    let connection = pool.get().map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;
    let input: i32 = info.0;
    let result =
        web::block(move || super::db::update_user_status(input, State::Leave, &connection))
            .await
            .map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            })?;
    if let Some(result) = result {
        Ok(HttpResponse::Ok().json(result))
    } else {
        let res = HttpResponse::NotFound().body(format!("No user found with id: {}", input));
        Ok(res)
    }
}

pub async fn get_all(pool: web::Data<DbPool>) -> Result<HttpResponse> {
    let connection = pool.get().map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;
    let ret = web::block(move || super::db::list_users(&connection))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;
    Ok(HttpResponse::Ok().json(ret))
}

pub async fn get_student(pool: web::Data<DbPool>, info: web::Path<(i32,)>) -> Result<HttpResponse> {
    let connection = pool.get().map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;
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
        let res = HttpResponse::NotFound().body(format!("No user found with id: {}", input));
        Ok(res)
    }
}

pub async fn register(pool: web::Data<DbPool>, info: web::Json<NewPerson>) -> Result<HttpResponse> {
    let connection = pool.get().map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    web::block(move || super::db::register_user(&connection, &info))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().finish())
}
