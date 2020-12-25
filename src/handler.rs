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

fn date(map: &HashMap<String, String>) -> UpdatedAt {
    if let Some(date) = map.get("date".into()) {
        UpdatedAt::from_str(date).sub(-1)
    } else {
        UpdatedAt::new()
    }
}

fn get_valid_token(query: &HashMap<String, String>) -> Result<Token<IdPayload>, Rejection> {
    let token = query
        .get("token")
        .ok_or(CustomError::TokenIsNotFound.rejection())?;
    let client = google_jwt_verify::Client::new(&config().google_oauth_client_id());
    tokio::task::block_in_place(move || client.verify_id_token(&token))
        .map_err(|_| CustomError::TokenIsInvalid.rejection())
}

fn get_profile(query: &HashMap<String, String>) -> Result<Profile, Rejection> {
    let id_token = get_valid_token(query)?;
    Ok(Profile {
        user_id: id_token.get_claims().get_subject(),
        email: id_token.get_payload().get_email(),
        name: id_token.get_payload().get_name(),
    })
}

fn get_account(query: &HashMap<String, String>) -> Result<Account, Rejection> {
    let profile = get_profile(&query)?;
    let repos = MySQLClient::new();
    repos
        .account(profile.email)
        .map_err(|_| CustomError::AccountIsNotFound.rejection())
}

#[derive(Clone, Debug)]
struct Profile {
    user_id: String,
    email: String,
    name: String,
}
