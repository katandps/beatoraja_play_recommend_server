use crate::Result;
use beatoraja_play_recommend::*;
use serde::Serialize;
use warp::{http::StatusCode, Reply};

pub async fn health() -> Result<impl Reply> {
    Ok(StatusCode::OK)
}

pub async fn tables(tables: Tables) -> Result<impl Reply> {
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

pub async fn lamp(tables: Tables, table_index: usize) -> Result<impl Reply> {
    Ok(graph(tables, table_index, Command::LampGraph))
}

pub async fn lamps(tables: Tables) -> Result<impl Reply> {
    Ok(graphs(tables, Command::LampGraph))
}

pub async fn rank(tables: Tables, table_index: usize) -> Result<impl Reply> {
    Ok(graph(tables, table_index, Command::RankGraph))
}

pub async fn ranks(tables: Tables) -> Result<impl Reply> {
    Ok(graphs(tables, Command::RankGraph))
}

pub async fn recommend(tables: Tables, table_index: usize) -> Result<impl Reply> {
    Ok(graph(tables, table_index, Command::Recommend))
}

pub async fn recommends(tables: Tables) -> Result<impl Reply> {
    Ok(graphs(tables, Command::Recommend))
}

pub async fn detail(tables: Tables, table_index: usize) -> Result<impl Reply> {
    Ok(graph(tables, table_index, Command::Detail))
}

pub async fn details(tables: Tables) -> Result<impl Reply> {
    Ok(graphs(tables, Command::Detail))
}

fn graph(tables: Tables, table_index: usize, command: Command) -> String {
    let repos = SqliteClient::new();
    let table = match tables.get(table_index) {
        Some(t) => t,
        None => tables.iter().next().unwrap(),
    }
    .clone();
    take(
        table,
        repos.song_data(),
        repos.score(),
        repos.score_log(),
        command,
    )
}

fn graphs(tables: Tables, command: Command) -> String {
    let repos = SqliteClient::new();
    let song_data = repos.song_data();
    let score_log = repos.score_log();
    format!(
        "[ {} ]",
        tables
            .iter()
            .map(|t| take(
                t.clone(),
                song_data.clone(),
                repos.score(),
                score_log.clone(),
                command
            ))
            .collect::<Vec<String>>()
            .join(",")
    )
}
