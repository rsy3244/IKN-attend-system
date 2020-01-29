use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use listenfd::ListenFd;

#[derive(Serialize, Deserialize)]
struct MyObj {
    name: String,
}

async fn index() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(MyObj {
        name: String::from("monkukui"),
    }))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    let mut listenfd = ListenFd::from_env();
    let mut server =  HttpServer::new(|| {
        App::new()
            .route("/api/test", web::to(index))
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:8080")?
    };

    server.run().await
}
