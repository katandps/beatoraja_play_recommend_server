use beatoraja_play_recommend::*;
use diesel::r2d2::ConnectionManager;
use diesel::{Connection, MysqlConnection};
use r2d2::Pool;
use serde::Serialize;
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

pub async fn health_handler(
    db_pool: Pool<ConnectionManager<MysqlConnection>>,
) -> std::result::Result<impl Reply, Rejection> {
    match db_pool.get() {
        Ok(db) => match db.execute("SELECT 1") {
            Ok(_) => Ok(StatusCode::OK),
            Err(_) => Ok(StatusCode::INTERNAL_SERVER_ERROR),
        },
        Err(_) => Ok(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn table_handler(tables: Tables) -> std::result::Result<impl Reply, Rejection> {
    Ok(serde_json::to_string(
        &tables
            .iter()
            .map(|t| TableFormat {
                name: t.name(),
                levels: t
                    .levels()
                    .iter()
                    .cloned()
                    .map(|l| format!("{}{}", t.symbol(), l.to_string()))
                    .collect::<Vec<_>>(),
            })
            .collect::<Vec<_>>(),
    )
    .unwrap())
}

#[derive(Serialize)]
struct TableFormat {
    name: String,
    levels: Vec<String>,
}

/// 詳細表示ハンドラ
/// user_idをQueryParameterより取得する
/// 未入力の場合は1になる
pub async fn detail_handler(
    tables: Tables,
    query: HashMap<String, String>,
) -> Result<impl Reply, Rejection> {
    let repos = MySQLClient::new();
    let user_id = query.get(&"user_id".to_string()).unwrap_or(&"1".to_string()).clone();
    let num_user_id = user_id.parse::<i32>();
    if num_user_id.is_err() {
        return Ok("{\"message\": \"user_id is invalid\"}".into());
    }
    let num_user_id = num_user_id.unwrap();
    let account = repos.account_by_id(num_user_id);
    if account.is_err() {
        return Ok("{\"message\": \"account is not found\"}".into());
    }
    let account = account.unwrap();
    Ok(format!(
        "[ {} ]",
        tables
            .iter()
            .map(|t| beatoraja_play_recommend::Controller::for_server(
                t.clone(),
                repos.song_data(),
                repos.score(account.clone()).unwrap(),
                Command::Detail,
            )
            .run(date(&query))
            .to_string())
            .collect::<Vec<String>>()
            .join(",")
    ))
}

pub async fn my_detail_handler(
    tables: Tables,
    query: HashMap<String, String>,
) -> Result<impl Reply, Rejection> {
    let repos = MySQLClient::new();
    let token = query.get("token".into());
    if token.is_none() {
        return Ok("{\"message\":\"token is not input\"}".into());
    }
    let profile = get_profile(token.unwrap());
    if profile.is_err() {
        return Ok("{\"message\":\"token is invalid\"}".into());
    }
    let profile = profile.unwrap();
    let account = repos.account(profile.email);
    if account.is_err() {
        return Ok("{\"message\": \"account is not found\"}".into());
    }
    let account = account.unwrap();
    Ok(format!(
        "[ {} ]",
        tables
            .iter()
            .map(|t| beatoraja_play_recommend::Controller::for_server(
                t.clone(),
                repos.song_data(),
                repos.score(account.clone()).unwrap(),
                Command::Detail,
            )
            .run(date(&query))
            .to_string())
            .collect::<Vec<String>>()
            .join(",")
    ))
}

pub async fn history_handler() -> std::result::Result<impl Reply, Rejection> {
    let repos = beatoraja_play_recommend::SqliteClient::new();
    Ok(serde_json::to_string(&repos.player().diff()).unwrap())
}

fn date(map: &HashMap<String, String>) -> UpdatedAt {
    if let Some(date) = map.get("date".into()) {
        UpdatedAt::from_str(date).sub(-1)
    } else {
        UpdatedAt::new()
    }
}

fn get_profile(token: &String) -> Result<Profile, google_jwt_verify::Error> {
    let client_id = config().google_oauth_client_id();
    let client = google_jwt_verify::Client::new(&client_id);
    let id_token = tokio::task::block_in_place(move || client.verify_id_token(&token))?;

    let user_id = id_token.get_claims().get_subject();
    let email = id_token.get_payload().get_email();
    let name = id_token.get_payload().get_name();
    Ok(Profile {
        user_id,
        email,
        name,
    })
}

#[derive(Clone, Debug)]
struct Profile {
    user_id: String,
    email: String,
    name: String,
}
