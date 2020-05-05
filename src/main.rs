use actix_web::middleware::Logger;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};

fn index(req: HttpRequest) -> HttpResponse {
    HttpResponse::from("Hello World")
}
fn main() {
    run("0.0.0.0:8000");
}

#[actix_rt::main]
async fn run(host: &'static str) -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(web::resource("/").to(index))
    })
    .bind(host)?
    .run()
    .await
}
