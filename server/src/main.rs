#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_web::web;
use listenfd::ListenFd;

pub mod api;
pub mod person;
pub mod schema;
pub mod db;


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};


    let mut listenfd = ListenFd::from_env();
    let mut server =  HttpServer::new(|| {
        App::new()
            .route("/api/attend/{id}", web::put().to(api::attend))
            .route("/api/leave/{id}", web::put().to(api::leave))
            .route("/api/students", web::get().to(api::get_all))
            .route("/api/student/{id}", web::get().to(api::get_student))
            //.route("/api/signup/", web::post().to(api::signup))
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:8080")?
    };

    server.run().await
}
