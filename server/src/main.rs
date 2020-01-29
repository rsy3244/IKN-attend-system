use actix_web::{web, get, App, HttpResponse, HttpServer, Responder};
use listenfd::ListenFd;

#[get("/index")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body(include_str!("../../client/dst/index.html"))
}

async fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello world again!")
}

#[get("/hello")]
async fn index3() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let mut listenfd = ListenFd::from_env();
    let mut server =  HttpServer::new(|| {
        App::new()
            .service(index)
            .route("/again", web::get().to(index2))
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:8080")?
    };

    server.run().await
}
