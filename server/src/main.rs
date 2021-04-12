#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_web::web;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use listenfd::ListenFd;

pub mod api;
pub mod db;
pub mod person;
pub mod schema;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};
    dotenv::dotenv().ok();

    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/api/attend/{id}", web::put().to(api::attend))
            .route("/api/leave/{id}", web::put().to(api::leave))
            .route("/api/students", web::get().to(api::get_all))
            .route("/api/student/{id}", web::get().to(api::get_student))
            .route("/api/signup/", web::post().to(api::register))
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:8080")?
    };

    server.run().await
}
