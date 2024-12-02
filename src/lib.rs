mod routes;

pub use routes::hello_world;
use routes::{hello_bird, minus_one};

pub fn router() -> axum::Router {
    use axum::routing::get;
    use axum::Router;

    Router::new()
        .route("/hello_world", get(hello_world))
        .route("/-1/seek", get(minus_one))
        .route("/", get(hello_bird))
}

#[cfg(test)]
mod test {
    use super::*;
    use axum::http::StatusCode;
    use axum_test::TestServer;

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
