use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
struct ColumnRaw {
    id : usize,
    username : String,
    role : String,
    state : String,
}

struct Column {
    id : usize,
    username : String,
    role : String,
    state : State,
}

impl Column {
    fn into_raw(&self) -> ColumnRaw {
        ColumnRaw {
            id: self.id,
            username: self.username.clone(),
            role: self.role.clone(),
            state: match self.state {
                State::Attend => String::from("1"),
                State::Leave => String::from("0"),
                _ => panic!("not implemented!"),
            }
        }
    }
}

#[non_exhaustive]
enum State {
    Attend,
    Leave,
}

pub async fn attend(info: web::Path<(usize,)>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(
            Column {
                id: info.0,
                username: "サザエさん".to_string(),
                role: "B4".to_string(),
                state: State::Attend,
            }.into_raw()
    ))
}

pub async fn leave(info: web::Path<(usize,)>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(
            Column {
                id: info.0,
                username:  "サザエさん".to_string(),
                role: "B4".to_string(),
                state: State::Leave,
            }.into_raw()
    ))
}

pub async fn get_all() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(
            vec![
                Column {
                    id: 1,
                    username: "カツオ".to_string(),
                    role: "Professor".to_string(),
                    state: State::Leave,
                }.into_raw()
                ; 3
            ]
    ))
}

pub async fn get_student(info: web::Path<(usize,)>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(
            Column {
                id: info.0,
                username: "波平".to_string(),
                role: "M1".to_string(),
                state: State::Attend,
            }.into_raw()
    ))
}
