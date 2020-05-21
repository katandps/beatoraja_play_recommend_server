use actix_web::middleware::Logger;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use beatoraja_play_recommend::*;

fn index(req: HttpRequest) -> HttpResponse {
    let client = mysql::MySQLClient::new();
    dbg!(client.score_log());
    HttpResponse::from(beatoraja_play_recommend::take())
}
fn main() {
    println!("Listen started at port 80");
    let res = run("0.0.0.0:80");
    println!("{:?}", res);
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
