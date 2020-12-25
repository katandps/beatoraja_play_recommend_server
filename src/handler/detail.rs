use beatoraja_play_recommend::{MySQLClient, SongRepository, Tables};
use std::collections::HashMap;
use warp::{Rejection, Reply};

/// 詳細表示ハンドラ
/// user_idをQueryParameterより取得する
pub async fn detail_handler(
    tables: Tables,
    query: HashMap<String, String>,
) -> Result<impl Reply, Rejection> {
    let repos = MySQLClient::new();
    let user_id = query
        .get(&"user_id".to_string())
        .unwrap_or(&"1".to_string())
        .clone();
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

    let songs = repos.song_data();
    let scores = repos.score(account).unwrap();
    let date = super::date(&query);
    Ok(tables.make_detail(&songs, &scores, &date))
}

pub async fn my_detail_handler(
    tables: Tables,
    query: HashMap<String, String>,
) -> Result<impl Reply, Rejection> {
    let account = super::get_account(&query)?;
    let repos = MySQLClient::new();

    let songs = repos.song_data();
    let scores = repos.score(account).unwrap();
    let date = super::date(&query);
    Ok(tables.make_detail(&songs, &scores, &date))
}
