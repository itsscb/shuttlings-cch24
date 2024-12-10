#[cfg(feature = "task19")]
mod task_nineteen {
    use axum_test::TestServer;
    use itsscb_shuttlings_cch24::router;

    fn test_server() -> TestServer {
        TestServer::new(router()).unwrap()
    }
}
