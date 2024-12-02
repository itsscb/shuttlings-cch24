use itsscb_shuttlings_cch24::router;

#[shuttle_runtime::main]
#[allow(clippy::unused_async)]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = router();

    Ok(router.into())
}
