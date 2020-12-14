mod error;
mod handler;

use std::env;

const TLS_CERT_PATH: &str = "TLS_CERT_PATH";
const TLS_CERT_PATH_DEFAULT: &str = "./files/cert.pem";
const TLS_KEY_PATH: &str = "TLS_KEY_PATH";
const TLS_KEY_PATH_DEFAULT: &str = "./files/key.rsa";

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    let (_http_addr, http_warp) =
        warp::serve(handler::routes().await).bind_ephemeral(([0, 0, 0, 0], 80));

    let (_https_addr, https_warp) = warp::serve(handler::routes().await)
        .tls()
        .cert_path(get_env(TLS_CERT_PATH, TLS_CERT_PATH_DEFAULT))
        .key_path(get_env(TLS_KEY_PATH, TLS_KEY_PATH_DEFAULT))
        .bind_ephemeral(([0, 0, 0, 0], 443));

    futures::future::join(http_warp, https_warp).await;
}

fn get_env(key: &str, default: &str) -> String {
    match env::var(key) {
        Ok(val) => val,
        Err(_) => default.into(),
    }
}
