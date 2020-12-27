pub mod detail;
pub mod health;
pub mod upload;

use crate::error::*;
use beatoraja_play_recommend::*;
use diesel::r2d2::ConnectionManager;
use diesel::MysqlConnection;
use google_jwt_verify::{IdPayload, Token};
use r2d2::Pool;
use std::collections::HashMap;
use std::convert::Infallible;
use warp::http::StatusCode;
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
