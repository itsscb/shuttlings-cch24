#[cfg(all(test, feature = "task1-9"))]
mod minus_one {

    use axum::http::StatusCode;
    use axum_test::TestServer;
    use itsscb_shuttlings_cch24::router;

    fn test_server() -> TestServer {
        TestServer::new(router()).unwrap()
    }

    #[tokio::test]
    async fn test_hello_world() {
        let server = test_server();

        let response = server.get("/hello_world").await;
        response.assert_status_ok();
        response.assert_text("Hello, world!");
    }

    #[tokio::test]
    async fn test_hello_bird() {
        let server = test_server();

        let response = server.get("/").await;
        response.assert_status_ok();
        response.assert_text("Hello, bird!");
    }

    #[tokio::test]
    async fn test_minus_one() {
        let server = test_server();

        let response = server.get("/-1/seek").await;
        response.assert_header("location", "https://www.youtube.com/watch?v=9Gc4QTqslN4");
        response.assert_status(StatusCode::FOUND);
    }
}
