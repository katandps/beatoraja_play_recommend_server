use crate::error::HandleError::{
    AccountIsNotFound, AccountIsNotSelected, AccountSelectionIsInvalid,
};
use beatoraja_play_recommend::{MySQLClient, Scores, Tables};
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
        .ok_or(AccountIsNotSelected.rejection())?;
    let user_id = user_id
        .parse::<i32>()
        .map_err(|_| AccountSelectionIsInvalid.rejection())?;
    let account = repos
        .account_by_increments(user_id)
        .map_err(|_| AccountIsNotFound.rejection())?;
    let songs = repos.song_data();
    let scores = repos.score(account).unwrap_or(Scores::new(HashMap::new()));
    let date = super::date(&query);
    Ok(tables.make_detail(&songs, &scores, &date))
}

pub async fn my_detail_handler(
    tables: Tables,
    session_key: String,
    query: HashMap<String, String>,
) -> Result<impl Reply, Rejection> {
    let repos = MySQLClient::new();
    let account = crate::session::get_account_by_session(&session_key)
        .map_err(|_| AccountIsNotFound.rejection())?;
    let songs = repos.song_data();
    dbg!(&account);
    let scores = repos.score(account).unwrap_or(Scores::new(HashMap::new()));
    let date = super::date(&query);
    Ok(tables.make_detail(&songs, &scores, &date))
}
