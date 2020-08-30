mod error;
mod handler;

use beatoraja_play_recommend::*;
use std::convert::Infallible;
use warp::Filter;

type Result<T> = std::result::Result<T, warp::Rejection>;

#[tokio::main]
async fn main() {
    let tables = get_tables().await;

    let health_route = warp::path!("health").and_then(handler::health);

    let lamp = warp::path("lamp")
        .and(warp::get())
        .and(with_table(tables.clone()));
    let table_list = lamp.clone().and_then(handler::tables);
    let lamp = lamp
        .and(warp::path::param())
        .and_then(handler::lamp)
        .or(table_list);

    let routes = health_route
        .or(lamp)
        .with(warp::cors().allow_any_origin())
        .recover(error::handle_rejection);

    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}

fn with_table(tables: Tables) -> impl Filter<Extract = (Tables,), Error = Infallible> + Clone {
    warp::any().map(move || tables.clone())
}
