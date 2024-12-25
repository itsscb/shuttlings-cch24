#[cfg(feature = "task19")]
mod task_nineteen {
    use axum::http::StatusCode;
    use axum_test::TestServer;
    use itsscb_shuttlings_cch24::router;
    use serde_json::json;
    use sqlx::postgres::PgPoolOptions;

    async fn test_server() -> TestServer {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://postgres:postgres@localhost/my_database")
            .await
            .unwrap();

        sqlx::migrate!()
            .run(&pool)
            .await
            .expect("Failed to migrate database");

        TestServer::new(router(Some(pool))).unwrap()
    }

    #[tokio::test]
    async fn test_reset() {
        let server = test_server().await;
        let response = server.post("/19/reset").await;
        response.assert_status_ok();
    }

    #[tokio::test]
    async fn test_draft() {
        let server = test_server().await;
        let response = server
            .post("/19/draft")
            .text(r#"{"author":"Santa","quote":"TEST QUOTE"}"#)
            .content_type("application/json")
            .await;
        response.assert_status(StatusCode::CREATED);

        let val = json!({
            "author": "Santa",
            "quote": "TEST QUOTE"
        });
        response.assert_json_contains(&val);
    }

    #[tokio::test]
    async fn test_cite() {
        let server = test_server().await;
        let response = server
            .post("/19/draft")
            .text(r#"{"author":"Santa","quote":"TEST QUOTE"}"#)
            .content_type("application/json")
            .await;
        response.assert_status(StatusCode::CREATED);

        let val = json!({
            "author": "Santa",
            "quote": "TEST QUOTE"
        });
        response.assert_json_contains(&val);

        let id = response.json::<serde_json::Value>()["id"]
            .as_str()
            .unwrap()
            .to_string();

        let response = server.get(&format!("/19/cite/{id}")).await;
        // dbg!(&response);

        response.assert_status_ok();

        let val = json!({
            "author": "Santa",
            "quote": "TEST QUOTE"
        });
        response.assert_json_contains(&val);

        let response = server.get("/19/cite/asdfasdf").await;
        response.assert_status_not_found();
    }

    #[tokio::test]
    // #[ignore]
    async fn test_list() {
        let server = test_server().await;
        let response = server
            .post("/19/draft")
            .text(r#"{"author":"Santa","quote":"TEST QUOTE"}"#)
            .content_type("application/json")
            .await;
        response.assert_status(StatusCode::CREATED);

        let val = json!({
            "author": "Santa",
            "quote": "TEST QUOTE"
        });
        response.assert_json_contains(&val);

        let response = server.get("/19/list?token=fadsfasdfasf").await;
        dbg!(&response);

        response.assert_status_ok();
    }

    #[tokio::test]
    async fn test_remove() {
        let server = test_server().await;
        let response = server
            .post("/19/draft")
            .text(r#"{"author":"Santa","quote":"TEST QUOTE"}"#)
            .content_type("application/json")
            .await;
        response.assert_status(StatusCode::CREATED);

        let val = json!({
            "author": "Santa",
            "quote": "TEST QUOTE"
        });
        response.assert_json_contains(&val);

        let id = response.json::<serde_json::Value>()["id"]
            .as_str()
            .unwrap()
            .to_string();

        let response = server.delete(&format!("/19/remove/{id}")).await;

        response.assert_status_ok();

        let val = json!({
            "author": "Santa",
            "quote": "TEST QUOTE"
        });
        response.assert_json_contains(&val);

        let response = server.delete("/19/remove/asdfasdf").await;
        response.assert_status_not_found();
    }

    #[tokio::test]
    async fn test_undo() {
        let server = test_server().await;
        let response = server
            .post("/19/draft")
            .text(r#"{"author":"Santa","quote":"TEST QUOTE"}"#)
            .content_type("application/json")
            .await;
        response.assert_status(StatusCode::CREATED);

        let val = json!({
            "author": "Santa",
            "quote": "TEST QUOTE"
        });
        response.assert_json_contains(&val);

        let id = response.json::<serde_json::Value>()["id"]
            .as_str()
            .unwrap()
            .to_string();
        let response = server
            .put(&format!("/19/undo/{id}"))
            .text(r#"{"author":"Santa","quote":"updated TEST QUOTE"}"#)
            .content_type("application/json")
            .await;
        response.assert_status_ok();

        let val = json!({
            "author": "Santa",
            "quote": "updated TEST QUOTE"
        });
        response.assert_json_contains(&val);

        let response = server
            .put(&format!("/19/undo/{id}"))
            .text(r#"{"author":"NOT SANTA","quote":""}"#)
            .content_type("application/json")
            .await;
        response.assert_status_ok();

        let val = json!({
            "author": "NOT SANTA",
            "quote": ""
        });
        response.assert_json_contains(&val);

        let response = server
            .put("/19/undo/asdfasdf")
            .text(r#"{"author":"Santa","quote":"updated TEST QUOTE"}"#)
            .content_type("application/json")
            .await;
        response.assert_status_bad_request();
    }
}
