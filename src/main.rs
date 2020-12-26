mod error;
mod handler;

use diesel::r2d2::ConnectionManager;
use diesel::MysqlConnection;
use r2d2::Pool;
use std::collections::HashMap;
use std::env;
use warp::Filter;

const TLS_CERT_PATH: &str = "TLS_CERT_PATH";
const TLS_CERT_PATH_DEFAULT: &str = "./files/cert.pem";
const TLS_KEY_PATH: &str = "TLS_KEY_PATH";
const TLS_KEY_PATH_DEFAULT: &str = "./files/key.rsa";

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();
    let log = warp::log("example");

    let db_pool = get_db_pool();

    let tables = beatoraja_play_recommend::get_tables(false).await;
    let tables_route = warp::get()
        .and(warp::path("tables"))
        .and(handler::with_table(tables.clone()))
        .and(warp::path::end())
        .and_then(handler::table_handler);

    let health_route = warp::get()
        .and(warp::path("health"))
        .and(handler::with_db(db_pool.clone()))
        .and_then(handler::health::health_handler);

    let account_route = warp::get()
        .and(warp::path("account"))
        .and(warp::query::<HashMap<String, String>>())
        .and_then(handler::account_handler);

    let my_detail_route = warp::get()
        .and(warp::path("my_detail"))
        .and(warp::path::end())
        .and(handler::with_table(tables.clone()))
        .and(warp::query::<HashMap<String, String>>())
        .and_then(handler::detail::my_detail_handler);

    let detail_route = warp::get()
        .and(warp::path("detail"))
        .and(warp::path::end())
        .and(handler::with_table(tables.clone()))
        .and(warp::query::<HashMap<String, String>>())
        .and_then(handler::detail::detail_handler);

    let history_route = warp::get()
        .and(warp::path("history"))
        .and_then(handler::history_handler);

    let score_upload_route = warp::post()
        .and(warp::path("upload"))
        .and(warp::path("score"))
        .and(warp::multipart::form().max_length(100 * 1024 * 1024))
        .and(warp::query::<HashMap<String, String>>())
        .and_then(handler::upload::upload_score_handler);

    let scorelog_upload_route = warp::post()
        .and(warp::path("upload"))
        .and(warp::path("score_log"))
        .and(warp::multipart::form().max_length(100 * 1024 * 1024))
        .and(warp::query::<HashMap<String, String>>())
        .and_then(handler::upload::upload_score_log_handler);

    let songdata_upload_route = warp::post()
        .and(warp::path("upload"))
        .and(warp::path("song_data"))
        .and(warp::multipart::form().max_length(100 * 1024 * 1024))
        .and(warp::query::<HashMap<String, String>>())
        .and_then(handler::upload::upload_song_data_handler);

    let route = health_route
        .or(account_route)
        .or(tables_route)
        .or(detail_route)
        .or(my_detail_route)
        .or(history_route)
        .or(score_upload_route)
        .or(scorelog_upload_route)
        .or(songdata_upload_route)
        .recover(error::handle_rejection)
        .with(
            warp::cors()
                .allow_any_origin()
                .allow_methods(vec!["GET", "POST", "OPTIONS"])
                .allow_headers(vec![
                    "x-requested-with",
                    "origin",
                    "referer",
                    "x-csrftoken",
                    "content-type",
                    "content-length",
                    "accept",
                    "accept-encoding",
                    "accept-language",
                    "user-agent",
                ]),
        )
        .with(log);

    let (_http_addr, http_warp) = warp::serve(route.clone()).bind_ephemeral(([0, 0, 0, 0], 8000));

    let (_https_addr, https_warp) = warp::serve(route.clone())
        .tls()
        .cert_path(get_env(TLS_CERT_PATH, TLS_CERT_PATH_DEFAULT))
        .key_path(get_env(TLS_KEY_PATH, TLS_KEY_PATH_DEFAULT))
        .bind_ephemeral(([0, 0, 0, 0], 4431));

    futures::future::join(http_warp, https_warp).await;
}

fn get_env(key: &str, default: &str) -> String {
    match env::var(key) {
        Ok(val) => val,
        Err(_) => default.into(),
    }
}

fn get_db_pool() -> Pool<ConnectionManager<MysqlConnection>> {
    Pool::builder().build_unchecked(ConnectionManager::new(
        beatoraja_play_recommend::config().mysql_url(),
    ))
}
