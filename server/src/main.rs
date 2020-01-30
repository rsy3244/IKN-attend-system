use actix_web::{web, HttpResponse, Result, Responder};
use serde::{Deserialize, Serialize};
use listenfd::ListenFd;

mod api;

#[derive(Serialize, Deserialize)]
struct MyObj {
    name: String,
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("hello")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    let mut listenfd = ListenFd::from_env();
    let mut server =  HttpServer::new(|| {
        App::new()
            .route("/api/attend/{id}", web::put().to(api::attend))
            .route("/api/leave/{id}", web::put().to(api::leave))
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:8080")?
    };

    server.run().await
}
