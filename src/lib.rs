mod routes;

use axum::routing::{delete, put};
use routes::task_nineteen::{cite, draft, list, remove, reset, undo};
#[cfg(feature = "task12")]
use routes::{board, place, random_board, reset, Board};
#[cfg(feature = "task16")]
use routes::{decode, unwrap, wrap};
#[cfg(feature = "task1-9")]
use routes::{
    hello_bird, hello_world, ipv4_dest, ipv4_key, ipv6_dest, ipv6_key, manifest, milk, minus_one,
    refill, MilkFactory,
};

#[allow(unused_imports)]
pub fn router(pool: Option<sqlx::PgPool>) -> axum::Router {
    use axum::{
        routing::{get, post},
        Router,
    };

    #[cfg(feature = "task1-9")]
    let milk_factory = MilkFactory::new();

    #[cfg(feature = "task1-9")]
    return Router::new()
        .route("/hello_world", get(hello_world))
        .route("/-1/seek", get(minus_one))
        .route("/2/dest", get(ipv4_dest))
        .route("/2/key", get(ipv4_key))
        .route("/2/v6/dest", get(ipv6_dest))
        .route("/2/v6/key", get(ipv6_key))
        .route("/5/manifest", post(manifest))
        .route("/9/milk", post(milk))
        .route("/9/refill", post(refill))
        .with_state(milk_factory)
        .route("/", get(hello_bird));

    #[cfg(feature = "task12")]
    Router::new()
        .route("/12/board", get(board))
        .route("/12/reset", post(reset))
        .route("/12/place/:team/:column", post(place))
        .route("/12/random-board", get(random_board))
        .with_state(Board::new());

    #[cfg(feature = "task16")]
    Router::new()
        .route("/16/wrap", post(wrap))
        .route("/16/unwrap", get(unwrap))
        .route("/16/decode", post(decode));

    pool.map_or_else(Router::new, |pool| {
        Router::new()
            .route("/19/reset", post(reset))
            .route("/19/draft", post(draft))
            .route("/19/undo/:id", put(undo))
            .route("/19/remove/:id", delete(remove))
            .route("/19/cite/:id", get(cite))
            .route("/19/list", get(list))
            .with_state(pool)
    })
}
