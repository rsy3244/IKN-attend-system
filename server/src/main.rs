use actix_web::{web, get, App, HttpResponse, HttpServer, Responder, Result};
use actix_files as fs;
use listenfd::ListenFd;

async fn index() -> Result<String> {
    Ok(String::from("Hello, world!"))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let mut listenfd = ListenFd::from_env();
    let mut server =  HttpServer::new(|| {
        App::new()
            .route("/test", web::to(index))
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:8080")?
    };

    server.run().await
}
