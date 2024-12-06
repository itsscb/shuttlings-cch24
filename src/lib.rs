mod routes;

use axum::routing::post;
pub use routes::hello_world;
use routes::{hello_bird, ipv4_dest, ipv4_key, ipv6_dest, ipv6_key, manifest, minus_one};

pub fn router() -> axum::Router {
    use axum::routing::get;
    use axum::Router;

    Router::new()
        .route("/hello_world", get(hello_world))
        .route("/-1/seek", get(minus_one))
        .route("/2/dest", get(ipv4_dest))
        .route("/2/key", get(ipv4_key))
        .route("/2/v6/dest", get(ipv6_dest))
        .route("/2/v6/key", get(ipv6_key))
        .route("/5/manifest", post(manifest))
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

    #[tokio::test]
    async fn test_ipv4_dest() {
        let server = test_server();

        let response = server.get("/2/dest?from=10.0.0.0&key=1.2.3.255").await;
        response.assert_status_ok();
        response.assert_text("11.2.3.255");

        let response = server.get("/2/dest?from=invalid-from&key=1.2.3.255").await;
        response.assert_status_bad_request();

        let response = server.get("/2/dest?from=10.0.0.0&key=invalid-key").await;
        response.assert_status_bad_request();

        let response = server
            .get("/2/dest?from=128.128.33.0&key=255.0.255.33")
            .await;
        response.assert_status_ok();
        response.assert_text("127.128.32.33");
    }

    #[tokio::test]
    async fn test_ipv4_key() {
        let server = test_server();

        let response = server.get("/2/key?from=10.0.0.0&to=11.2.3.255").await;
        response.assert_status_ok();
        response.assert_text("1.2.3.255");

        let response = server.get("/2/key?from=invalid-from&to=1.2.3.255").await;
        response.assert_status_bad_request();

        let response = server.get("/2/key?from=10.0.0.0&to=invalid-to").await;
        response.assert_status_bad_request();

        let response = server
            .get("/2/key?from=128.128.33.0&to=127.128.32.33")
            .await;
        response.assert_status_ok();
        response.assert_text("255.0.255.33");
    }

    #[tokio::test]
    async fn test_ipv6_dest() {
        let server = test_server();

        let response = server.get("/2/v6/dest?from=fe80::1&key=5:6:7::3333").await;
        response.assert_status_ok();
        response.assert_text("fe85:6:7::3332");

        let response = server
            .get("/2/v6/dest?from=invalid-from&key=5:6:7::3333")
            .await;
        response.assert_status_bad_request();

        let response = server.get("/2/v6/dest?from=fe80::1&key=invalid-key").await;
        response.assert_status_bad_request();
    }

    #[tokio::test]
    async fn test_ipv6_key() {
        let server = test_server();

        let response = server
            .get("/2/v6/key?from=aaaa::aaaa&to=5555:ffff:c:0:0:c:1234:5555")
            .await;
        response.assert_status_ok();
        response.assert_text("ffff:ffff:c::c:1234:ffff");

        let response = server
            .get("/2/v6/dest?from=invalid-from&to=5:6:7::3333")
            .await;
        response.assert_status_bad_request();

        let response = server.get("/2/v6/dest?from=fe80::1&to=invalid-to").await;
        response.assert_status_bad_request();
    }

    #[tokio::test]
    async fn test_manifest_1() {
        let server = test_server();

        let want = "Toy car: 2\nLego brick: 230";

        let payload = r#"[package]
name = "not-a-gift-order"
authors = ["Not Santa"]
keywords = ["Christmas 2024"]

[[package.metadata.orders]]
item = "Toy car"
quantity = 2

[[package.metadata.orders]]
item = "Lego brick"
quantity = 230"#;
        let response = server
            .post("/5/manifest")
            .text(payload)
            .content_type("application/toml")
            .await;

        response.assert_status_ok();
        response.assert_text(want);

        let payload = r#"[package]
name = "coal-in-a-bowl"
authors = ["H4CK3R_13E7"]
keywords = ["Christmas 2024"]

[[package.metadata.orders]]
item = "Coal"
quantity = "Hahaha get rekt""#;

        let response = server
            .post("/5/manifest")
            .text(payload)
            .content_type("application/toml")
            .await;
        response.assert_status(StatusCode::NO_CONTENT);
    }

    #[tokio::test]
    async fn test_manifest_2() {
        let server = test_server();

        let payload = r#"[package]
name = false
authors = ["Not Santa"]
keywords = ["Christmas 2024"]"#;

        let response = server
            .post("/5/manifest")
            .text(payload)
            .content_type("application/toml")
            .await;
        response.assert_status(StatusCode::BAD_REQUEST);

        let payload = r#"[package]
name = "not-a-gift-order"
authors = ["Not Santa"]
keywords = ["Christmas 2024"]

[profile.release]
incremental = "stonks""#;
        let response = server
            .post("/5/manifest")
            .text(payload)
            .content_type("application/toml")
            .await;
        response.assert_status(StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_manifest_3() {
        let server = test_server();

        let want = "Toy car: 2\nLego brick: 230";

        let payload = r#"[package]
name = "not-a-gift-order"
authors = ["Not Santa"]
keywords = ["Christmas 2024"]

[[package.metadata.orders]]
item = "Toy car"
quantity = 2

[[package.metadata.orders]]
item = "Lego brick"
quantity = 230"#;
        let response = server
            .post("/5/manifest")
            .text(payload)
            .content_type("application/toml")
            .await;

        response.assert_status_ok();
        response.assert_text(want);

        let payload = r#"[package]
name = "not-a-gift-order"
authors = ["Not Santa"]

[[package.metadata.orders]]
item = "Toy car"
quantity = 2

[[package.metadata.orders]]
item = "Lego brick"
quantity = 230"#;
        let response = server
            .post("/5/manifest")
            .text(payload)
            .content_type("application/toml")
            .await;

        response.assert_status(StatusCode::BAD_REQUEST);
        response.assert_text("Magic keyword not provided");

        let payload = r#"[package]
name = "grass"
authors = ["A vegan cow"]
keywords = ["Moooooo"]"#;

        let response = server
            .post("/5/manifest")
            .text(payload)
            .content_type("application/toml")
            .await;
        response.assert_status(StatusCode::BAD_REQUEST);
        response.assert_text("Magic keyword not provided");

        let payload = r#"[package]
name = "grass"
authors = ["A vegan cow"]
keywords = ["Christmas 2024"]"#;

        let response = server
            .post("/5/manifest")
            .text(payload)
            .content_type("application/toml")
            .await;
        response.assert_status(StatusCode::NO_CONTENT);
    }

    #[tokio::test]
    async fn test_manifest_4() {
        let server = test_server();

        let payload = r#"[package]
name = "grass"
authors = ["A vegan cow"]
keywords = ["Christmas 2024"]"#;

        let response = server
            .post("/5/manifest")
            .text(payload)
            .content_type("text/html")
            .await;
        response.assert_status(StatusCode::UNSUPPORTED_MEDIA_TYPE);

        let want = "Toy train: 5";

        let payload = r#"package:
  name: big-chungus-sleigh
  version: "2.0.24"
  metadata:
    orders:
      - item: "Toy train"
        quantity: 5
  rust-version: "1.69"
  keywords:
    - "Christmas 2024""#;

        let response = server
            .post("/5/manifest")
            .text(payload)
            .content_type("application/yaml")
            .await;

        response.assert_status_ok();
        response.assert_text(want);

        let want = "Toy train: 5";

        let payload = r#"{
    "package": {
        "name": "big-chungus-sleigh",
        "version": "2.0.24",
        "metadata": {
            "orders": [
                {
                    "item": "Toy train",
                    "quantity": 5
                }
            ]
        },
        "rust-version": "1.69",
        "keywords": [
            "Christmas 2024"
        ]
    }
}"#;

        let response = server
            .post("/5/manifest")
            .text(payload)
            .content_type("application/json")
            .await;

        response.assert_status_ok();
        response.assert_text(want);
    }
}
