use crate::Result;
use beatoraja_play_recommend::*;
use warp::{http::StatusCode, Reply};

pub async fn health() -> Result<impl Reply> {
    Ok(StatusCode::OK)
}

pub async fn tables(tables: Tables) -> Result<impl Reply> {
    let res: String = tables
        .iter()
        .map(|t| t.to_string())
        .collect::<Vec<String>>()
        .join("\n");
    Ok(warp::reply::html(res))
}

pub async fn lamp(tables: Tables, table_index: usize) -> Result<impl Reply> {
    let repos = SqliteClient::new();
    let table = match tables.get(table_index) {
        Some(t) => t,
        None => tables.iter().next().unwrap(),
    }
    .clone();
    let res = take(
        table,
        repos.song_data(),
        repos.score_log(),
        Command::LampGraph,
    );
    Ok(warp::reply::html(res))
}
