use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};

use super::person::*;


pub async fn attend(info: web::Path<(usize,)>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(
            Person::new(info.0, "サザエさん".to_string(), Role::B4, State::Leave)
            .to_raw()
    ))
}

pub async fn leave(info: web::Path<(usize,)>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(
            Person::new(info.0, "サザエさん".to_string(), Role::B4, State::Leave)
            .to_raw()
    ))
}

pub async fn get_all() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(
            vec![
                Person::new(1, "カツオ".to_string(), Role::Professor, State::Leave)
                .to_raw()
                ; 3
            ]
    ))
}

pub async fn get_student(info: web::Path<(usize,)>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(
            Person::new(info.0, "波平".to_string(), Role::M1, State::Attend)
            .to_raw()
    ))
}
