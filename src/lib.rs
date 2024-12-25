mod routes;

#[cfg(feature = "task19")]
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
use tower_http::services::ServeDir;

#[allow(unused_imports)]
use axum::{
    routing::{delete, get, post, put},
    Router,
};
#[cfg(feature = "task19")]
#[allow(unused_imports)]
pub fn router(pool: Option<sqlx::PgPool>) -> axum::Router {
    return pool.map_or_else(Router::new, |pool| {
        Router::new()
            .route("/19/reset", post(reset))
            .route("/19/draft", post(draft))
            .route("/19/undo/:id", put(undo))
            .route("/19/remove/:id", delete(remove))
            .route("/19/cite/:id", get(cite))
            .route("/19/list", get(list))
            .with_state(pool)
    });
}

#[cfg(not(feature = "task19"))]
pub fn router() -> axum::Router {
    use routes::task_twentythree::{lockfile, ornament, present, star};

    #[cfg(feature = "task1-9")]
    let milk_factory = MilkFactory::new();

    #[cfg(feature = "task1-9")]
    return return Router::new()
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
    return Router::new()
        .route("/12/board", get(board))
        .route("/12/reset", post(reset))
        .route("/12/place/:team/:column", post(place))
        .route("/12/random-board", get(random_board))
        .with_state(Board::new());

    #[cfg(feature = "task16")]
    return Router::new()
        .route("/16/wrap", post(wrap))
        .route("/16/unwrap", get(unwrap))
        .route("/16/decode", post(decode));

    Router::new()
        .route("/23/star", get(star))
        .route("/23/lockfile", post(lockfile))
        .route("/23/present/:color", get(present))
        .route("/23/ornament/:state/:id", get(ornament))
        .nest_service("/assets/", ServeDir::new("assets"))
}
