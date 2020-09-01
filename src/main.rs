mod error;
mod handler;

use beatoraja_play_recommend::*;
use std::convert::Infallible;
use std::env;
use warp::Filter;

type Result<T> = std::result::Result<T, warp::Rejection>;

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    let tables = get_tables().await;

    let health_route = warp::path!("health").and_then(handler::health);

    let tables_route = warp::path("tables")
        .and(warp::get())
        .and(with_table(tables.clone()))
        .and_then(handler::tables);

    let lamp_route = {
        let lamp = warp::path("lamp")
            .and(warp::get())
            .and(with_table(tables.clone()));
        let lamps_route = lamp.clone().and_then(handler::lamps);

        lamp.and(warp::path::param())
            .and_then(handler::lamp)
            .or(lamps_route)
    };

    let rank_route = {
        let rank = warp::path("rank")
            .and(warp::get())
            .and(with_table(tables.clone()));
        let ranks_route = rank.clone().and_then(handler::ranks);

        rank.and(warp::path::param())
            .and_then(handler::rank)
            .or(ranks_route)
    };

    let recommend_route = {
        let recommend = warp::path("recommend")
            .and(warp::get())
            .and(with_table(tables.clone()));
        let recommends_route = recommend.clone().and_then(handler::recommends);
        recommend
            .and(warp::path::param())
            .and_then(handler::recommend)
            .or(recommends_route)
    };

    let detail_route = warp::path("detail")
        .and(warp::get())
        .and(with_table(tables.clone()))
        .and(warp::path::param())
        .and_then(handler::detail);

    let routes = health_route
        .or(tables_route)
        .or(lamp_route)
        .or(rank_route)
        .or(recommend_route)
        .or(detail_route)
        .with(warp::cors().allow_any_origin())
        .recover(error::handle_rejection);

    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}

fn with_table(tables: Tables) -> impl Filter<Extract = (Tables,), Error = Infallible> + Clone {
    warp::any().map(move || tables.clone())
}
