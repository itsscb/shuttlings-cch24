#[cfg(all(test, feature = "task1-9"))]
mod task_five {
    use std::thread;
    use std::time::Duration;

    use axum::http::StatusCode;
    use axum_test::TestServer;
    use itsscb_shuttlings_cch24::router;

    fn test_server() -> TestServer {
        TestServer::new(router()).unwrap()
    }

    #[tokio::test]
    async fn test_milk() {
        let sever = test_server();

        let response = sever.post("/9/milk").await;
        response.assert_status_ok();
        response.assert_text("Milk withdrawn\n");
        let response = sever.post("/9/milk").await;
        response.assert_status_ok();
        response.assert_text("Milk withdrawn\n");
        let response = sever.post("/9/milk").await;
        response.assert_status_ok();
        response.assert_text("Milk withdrawn\n");
        let response = sever.post("/9/milk").await;
        response.assert_status_ok();
        response.assert_text("Milk withdrawn\n");
        let response = sever.post("/9/milk").await;
        response.assert_status_ok();
        response.assert_text("Milk withdrawn\n");
        let response = sever.post("/9/milk").await;
        response.assert_status(StatusCode::TOO_MANY_REQUESTS);

        let sever = test_server();

        for i in 0..=10 {
            let response = sever.post("/9/milk").await;
            match i {
                0..=4 | 6 | 8 | 9 => {
                    response.assert_status_ok();
                    response.assert_text("Milk withdrawn\n");
                }
                5 | 7 | 10 => {
                    response.assert_status(StatusCode::TOO_MANY_REQUESTS);
                    response.assert_text("No milk available\n");
                    match i {
                        5 => thread::sleep(Duration::from_secs(1)),
                        7 => thread::sleep(Duration::from_secs(2)),
                        _ => (),
                    }
                }
                _ => {
                    response.assert_status(StatusCode::SERVICE_UNAVAILABLE);
                    response.assert_text("No milk available\n");
                }
            }
        }

        let sever = test_server();

        let response = sever
            .post("/9/milk")
            .text(r#"{"liters":5}"#)
            .content_type("application/json")
            .await;
        response.assert_status_ok();
        response.assert_text(r#"{"gallons":1.3208603}"#);

        let sever = test_server();

        let response = sever
            .post("/9/milk")
            .text(r#"{"gallons":5}"#)
            .content_type("application/json")
            .await;

        response.assert_status_ok();
        response.assert_text(r#"{"liters":18.927061}"#);

        let sever = test_server();

        let response = sever
            .post("/9/milk")
            .text(r#"{"liters":1, "gallons":5}"#)
            .content_type("application/json")
            .await;
        response.assert_status_bad_request();

        let response = sever
            .post("/9/milk")
            .text(r#"{"litres":2}"#)
            .content_type("application/json")
            .await;
        response.assert_status_ok();
        response.assert_text(r#"{"pints":3.519508}"#);

        let sever = test_server();

        for i in 0..=11 {
            let response = sever.post("/9/milk").await;
            match i {
                0..=4 | 6..=10 => {
                    response.assert_status_ok();
                    response.assert_text("Milk withdrawn\n");
                }
                5 | 11 => {
                    response.assert_status(StatusCode::TOO_MANY_REQUESTS);
                    response.assert_text("No milk available\n");
                    let response = sever.post("/9/refill").await;
                    response.assert_status_ok();
                }
                _ => {
                    response.assert_status(StatusCode::SERVICE_UNAVAILABLE);
                    response.assert_text("No milk available\n");
                }
            }
        }
    }
}
