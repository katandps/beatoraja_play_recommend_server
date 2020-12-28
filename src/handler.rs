pub mod detail;
pub mod health;
pub mod upload;

use crate::error::*;
use beatoraja_play_recommend::config;
use beatoraja_play_recommend::*;
use diesel::r2d2::ConnectionManager;
use diesel::MysqlConnection;
use google_jwt_verify::{IdPayload, Token};
use r2d2::Pool;
use std::collections::HashMap;
use std::convert::Infallible;
use warp::http::{StatusCode, Uri};
use warp::{Filter, Rejection, Reply};

pub fn with_db(
    db_pool: Pool<ConnectionManager<MysqlConnection>>,
) -> impl Filter<Extract = (Pool<ConnectionManager<MysqlConnection>>,), Error = Infallible> + Clone
{
    warp::any().map(move || db_pool.clone())
}

pub fn with_table(tables: Tables) -> impl Filter<Extract = (Tables,), Error = Infallible> + Clone {
    warp::any().map(move || tables.clone())
}

pub async fn table_handler(tables: Tables) -> std::result::Result<impl Reply, Rejection> {
    Ok(serde_json::to_string(&tables.format()).unwrap())
}

pub async fn history_handler() -> std::result::Result<impl Reply, Rejection> {
    let repos = beatoraja_play_recommend::SqliteClient::by_config();
    Ok(serde_json::to_string(&repos.player().diff()).unwrap())
}

pub async fn account_handler(token: String) -> Result<impl Reply, Rejection> {
    get_account(token).map(|_| StatusCode::OK)
}

pub async fn oauth(query: HashMap<String, String>) -> Result<impl Reply, Rejection> {
    let code = query
        .get(&"code".to_string())
        .cloned()
        .ok_or(CustomError::CodeIsNotFound.rejection())?;
    let mut body = HashMap::new();
    body.insert("client_id", config().google_oauth_client_id());
    body.insert("client_secret", config().google_oauth_client_secret());
    body.insert("redirect_uri", config().google_oauth_redirect_uri());
    body.insert("code", code.clone());
    body.insert("grant_type", "authorization_code".to_string());
    let res = reqwest::Client::new()
        .post("https://accounts.google.com/o/oauth2/token")
        .json(&body)
        .send()
        .await
        .map_err(|_| CustomError::GoogleEndPointIsDown.rejection())?;
    let body = res
        .text()
        .await
        .map_err(|_| CustomError::GoogleResponseIsInvalid.rejection())?;
    let json: serde_json::Value = serde_json::from_str(&body)
        .map_err(|_| CustomError::GoogleResponseIsInvalid.rejection())?;
    let obj = json.as_object().unwrap();

    // todo ここでのみアカウントを作成する
    // todo sessionに入れるランダムキーを作る expireも作る

    let key = format!("key={}", "This is session key. Set a enough random key.");
    let uri = Uri::from_maybe_shared(format!("{}/home", config().client_url())).unwrap();
    let redirect = warp::redirect(uri);
    let redirect = warp::reply::with_header(redirect, http::header::SET_COOKIE, key);
    Ok(redirect)
}

fn date(map: &HashMap<String, String>) -> UpdatedAt {
    if let Some(date) = map.get("date".into()) {
        UpdatedAt::from_str(date).sub(-1)
    } else {
        UpdatedAt::new()
    }
}

fn get_valid_token(token: String) -> Result<Token<IdPayload>, Rejection> {
    let client = google_jwt_verify::Client::new(&config().google_oauth_client_id());
    tokio::task::block_in_place(|| client.verify_id_token(&token)).map_err(|e| {
        dbg!(&e, &token);
        CustomError::TokenIsInvalid.rejection()
    })
}

fn get_profile(token: String) -> Result<GoogleProfile, Rejection> {
    let id_token = get_valid_token(token)?;
    Ok(GoogleProfile {
        user_id: id_token.get_claims().get_subject(),
        email: id_token.get_payload().get_email(),
        name: id_token.get_payload().get_name(),
    })
}

fn get_account(token: String) -> Result<Account, Rejection> {
    let profile = get_profile(token)?;
    let repos = MySQLClient::new();
    repos
        .account(&profile)
        .map_err(|_| CustomError::AccountIsNotFound.rejection())
}
