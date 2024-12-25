use itsscb_shuttlings_cch24::router;

#[cfg(feature = "task12")]
#[shuttle_runtime::main]
#[allow(clippy::unused_async)]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = router();

    Ok(router.into())
}

#[cfg(feature = "task19")]
#[shuttle_runtime::main]
#[allow(clippy::unused_async)]
async fn main(#[shuttle_shared_db::Postgres] pool: sqlx::PgPool) -> shuttle_axum::ShuttleAxum {
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to migrate database");

    let router = router(Some(pool));

    Ok(router.into())
}

#[cfg(not(feature = "task19"))]
#[shuttle_runtime::main]
#[allow(clippy::unused_async)]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = router();

    Ok(router.into())
}
