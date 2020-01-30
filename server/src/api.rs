use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct ColumnRaw {
    id : usize,
    username : String,
    grade : String,
    state : String,
}

struct Column {
    id : usize,
    username : String,
    grade : String,
    state : State,
}

impl Column {
    fn into_raw(&self) -> ColumnRaw {
        ColumnRaw {
            id: self.id,
            username: self.username.clone(),
            grade: self.grade.clone(),
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
    println!("query attend");
    Ok(HttpResponse::Ok().json(
            Column {
                id: info.0,
                username: "サザエさん".to_string(),
                grade: "B4".to_string(),
                state: State::Attend,
            }.into_raw()
    ))
}

pub async fn leave(info: web::Path<(usize,)>) -> Result<HttpResponse> {
    println!("query leave");
    Ok(HttpResponse::Ok().json(
            Column {
                id: info.0,
                username:  "サザエさん".to_string(),
                grade: "B4".to_string(),
                state: State::Leave,
            }.into_raw()
    ))
}

//async fn index(info: web::Json<Column>) -> Result<String> {
