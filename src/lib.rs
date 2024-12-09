mod routes;

#[cfg(feature = "task1-9")]
use routes::{
    hello_bird, hello_world, ipv4_dest, ipv4_key, ipv6_dest, ipv6_key, manifest, milk, minus_one,
    refill, MilkFactory,
};

pub fn router() -> axum::Router {
    #[cfg(feature = "task1-9")]
    use axum::routing::get;
    #[cfg(feature = "task1-9")]
    use axum::routing::post;
    use axum::Router;

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

    // #[cfg(feature="task12")]
    Router::new()
}
