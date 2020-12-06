use crate::error;
use beatoraja_play_recommend::*;
use serde::Serialize;
use std::collections::HashMap;
use std::convert::Infallible;
use warp::http::StatusCode;
use warp::{filters::BoxedFilter, Filter, Reply};

pub async fn routes() -> BoxedFilter<(impl Reply,)> {
    let t = get_tables().await;

    health_route()
        .or(graphs_route(t.clone(), "lamp".into(), Command::LampGraph))
        .or(graphs_route(t.clone(), "rank".into(), Command::RankGraph))
        .or(graphs_route(t.clone(), "detail".into(), Command::Detail))
        .or(graph_route(t.clone(), "lamp".into(), Command::LampGraph))
        .or(graph_route(t.clone(), "rank".into(), Command::RankGraph))
        .or(graph_route(t.clone(), "detail".into(), Command::Detail))
        .or(tables_route(t.clone()))
        .or(history_route())
        .with(warp::cors().allow_any_origin())
        .recover(error::handle_rejection)
        .boxed()
}

fn health_route() -> BoxedFilter<(impl Reply,)> {
    warp::path("health")
        .map(|| warp::reply::with_status(warp::reply::json(&[123]), StatusCode::OK))
        .boxed()
}

fn graphs_route(tables: Tables, name: String, command: Command) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(warp::path(name))
        .and(warp::path::end())
        .and(with_table(tables))
        .and(warp::query::<HashMap<String, String>>())
        .map(move |tables, query| Ok(graphs(tables, command, date(&query))))
        .boxed()
}

fn graph_route(tables: Tables, name: String, command: Command) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(warp::path(name))
        .and(warp::path::param())
        .and(warp::path::end())
        .and(with_table(tables))
        .and(warp::query::<HashMap<String, String>>())
        .map(move |index, tables, query| Ok(graph(tables, index, command, date(&query))))
        .boxed()
}

fn tables_route(tables: Tables) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(warp::path("tables"))
        .and(with_table(tables))
        .and(warp::path::end())
        .map(|tables: Tables| {
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
        })
        .boxed()
}

fn history_route() -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(warp::path("history"))
        .map(|| {
            let repos = SqliteClient::new();
            Ok(serde_json::to_string(&repos.player().diff()).unwrap())
        })
        .boxed()
}

#[derive(Serialize)]
struct TableFormat {
    name: String,
    levels: Vec<String>,
}

fn graph(tables: Tables, table_index: usize, command: Command, date: UpdatedAt) -> String {
    let repos = SqliteClient::new();
    let table = match tables.get(table_index) {
        Some(t) => t,
        None => tables.iter().next().unwrap(),
    }
    .clone();
    take(table, repos.song_data(), repos.score(), command, date)
}

fn graphs(tables: Tables, command: Command, date: UpdatedAt) -> String {
    let repos = SqliteClient::new();
    let song_data = repos.song_data();
    format!(
        "[ {} ]",
        tables
            .iter()
            .map(|t| take(
                t.clone(),
                song_data.clone(),
                repos.score(),
                command,
                date.clone()
            ))
            .collect::<Vec<String>>()
            .join(",")
    )
}

fn with_table(tables: Tables) -> impl Filter<Extract = (Tables,), Error = Infallible> + Clone {
    warp::any().map(move || tables.clone())
}

fn date(map: &HashMap<String, String>) -> UpdatedAt {
    if let Some(date) = map.get("date".into()) {
        UpdatedAt::from_str(date).sub(-1)
    } else {
        UpdatedAt::new()
    }
}
