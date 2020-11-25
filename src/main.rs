mod error;
mod handler;

use beatoraja_play_recommend::*;
use std::convert::Infallible;
use std::env;
use warp::Filter;

type Result<T> = std::result::Result<T, warp::Rejection>;

const TLS_CERT_PATH: &str = "TLS_CERT_PATH";
const TLS_CERT_PATH_DEFAULT: &str = "./files/cert.pem";
const TLS_KEY_PATH: &str = "TLS_KEY_PATH";
const TLS_KEY_PATH_DEFAULT: &str = "./files/key.rsa";

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    let tables = get_tables().await;

    let health_route = warp::path!("health").and_then(handler::health);

    let tables_route = warp::path("tables")
        .and(warp::get())
        .and(with_table(tables.clone()))
        .and_then(handler::tables);

    let lamp_route = {
        let lamp = warp::path("lamp")
            .and(warp::get())
            .and(with_table(tables.clone()));
        let lamps_route = lamp.clone().and_then(handler::lamps);

        lamp.and(warp::path::param())
            .and_then(handler::lamp)
            .or(lamps_route)
    };

    let rank_route = {
        let rank = warp::path("rank")
            .and(warp::get())
            .and(with_table(tables.clone()));
        let ranks_route = rank.clone().and_then(handler::ranks);

        rank.and(warp::path::param())
            .and_then(handler::rank)
            .or(ranks_route)
    };

    let detail_route = {
        let detail = warp::path("detail")
            .and(warp::get())
            .and(with_table(tables.clone()));
        let details_route = detail.clone().and_then(handler::details);
        detail
            .and(warp::path::param())
            .and_then(handler::detail)
            .or(details_route)
    };

    let history_route = warp::path("history")
        .and(warp::get())
        .and_then(handler::history);

    let routes = health_route
        .or(tables_route)
        .or(lamp_route)
        .or(rank_route)
        .or(detail_route)
        .or(history_route)
        .with(warp::cors().allow_any_origin())
        .recover(error::handle_rejection);

    warp::serve(routes)
        .tls()
        .cert_path(get_env(TLS_CERT_PATH, TLS_CERT_PATH_DEFAULT))
        .key_path(get_env(TLS_KEY_PATH, TLS_KEY_PATH_DEFAULT))
        .run(([0, 0, 0, 0], 8000))
        .await;
}

fn get_env(key: &str, default: &str) -> String {
    match env::var(key) {
        Ok(val) => val,
        Err(_) => default.into(),
    }
}

fn with_table(tables: Tables) -> impl Filter<Extract = (Tables,), Error = Infallible> + Clone {
    warp::any().map(move || tables.clone())
}
