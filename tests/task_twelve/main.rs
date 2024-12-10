#[cfg(feature = "task12")]
mod task_twelve {
    use axum_test::TestServer;
    use itsscb_shuttlings_cch24::router;

    fn test_server() -> TestServer {
        TestServer::new(router()).unwrap()
    }
}
