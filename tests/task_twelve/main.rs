#![cfg(feature = "task12")]
#[cfg(test)]
mod task_twelve {
    use axum_test::TestServer;
    use itsscb_shuttlings_cch24::router;

    fn test_server() -> TestServer {
        TestServer::new(router()).unwrap()
    }
    const EMPTY_BOARD: &str = "⬜⬛⬛⬛⬛⬜
⬜⬛⬛⬛⬛⬜
⬜⬛⬛⬛⬛⬜
⬜⬛⬛⬛⬛⬜
⬜⬜⬜⬜⬜⬜
";

    #[tokio::test]
    async fn test_task_1() {
        let server = test_server();

        let response = server.get("/12/board").await;
        response.assert_status_ok();
        response.assert_text(EMPTY_BOARD);

        let response = server.post("/12/reset").await;
        response.assert_status_ok();
        response.assert_text(EMPTY_BOARD);
    }

    #[tokio::test]
    async fn test_task_2() {
        let server = test_server();
        let response = server.post("/12/reset").await;
        response.assert_status_ok();
        response.assert_text(EMPTY_BOARD);

        let want = "\
⬜⬛⬛⬛⬛⬜
⬜⬛⬛⬛⬛⬜
⬜⬛⬛⬛⬛⬜
⬜🍪⬛⬛⬛⬜
⬜⬜⬜⬜⬜⬜
";

        let response = server.post("/12/place/cookie/1").await;
        response.assert_status_ok();
        response.assert_text(want);

        let want = "\
⬜🍪⬛⬛⬛⬜
⬜🍪⬛⬛⬛⬜
⬜🍪⬛⬛⬛⬜
⬜🍪⬛⬛⬛⬜
⬜⬜⬜⬜⬜⬜
🍪 wins!
";

        let response = server.post("/12/place/cookie/1").await;
        response.assert_status_ok();

        let response = server.post("/12/place/cookie/1").await;
        response.assert_status_ok();
        let response = server.post("/12/place/cookie/1").await;
        response.assert_status_ok();
        response.assert_text(want);

        let response = server.post("/12/place/milk/2").await;
        response.assert_status_service_unavailable();
        response.assert_text(want);

        let mut response = server.post("/12/reset").await;
        response.assert_status_ok();
        response.assert_text(EMPTY_BOARD);

        let want = "⬜🥛🍪🥛🍪⬜
⬜🍪🥛🍪🥛⬜
⬜🍪🥛🍪🥛⬜
⬜🍪🥛🍪🥛⬜
⬜⬜⬜⬜⬜⬜
No winner.
";

        for i in 1..5 {
            for _ in 0..3 {
                let slot = if i % 2 == 0 { "milk" } else { "cookie" };
                response = server.post(&format!("/12/place/{slot}/{i}")).await;
                response.assert_status_ok();
            }
        }
        for i in 1..5 {
            let slot = if i % 2 == 0 { "cookie" } else { "milk" };

            response = server.post(&format!("/12/place/{slot}/{i}")).await;
            response.assert_status_ok();
        }
        response.assert_text(want);

        let response = server.post("/12/place/milk/1").await;
        response.assert_status_service_unavailable();
        response.assert_text(want);

        let response = server.post("/12/reset").await;
        response.assert_status_ok();
        response.assert_text(EMPTY_BOARD);

        let want = "⬜⬛⬛⬛🍪⬜
⬜⬛⬛🍪🥛⬜
⬜⬛🍪🥛🥛⬜
⬜🍪🥛🥛🥛⬜
⬜⬜⬜⬜⬜⬜
🍪 wins!
";

        let response = server.post("/12/place/cookie/1").await;
        response.assert_status_ok();

        for i in 2..5 {
            let response = server.post(&format!("/12/place/milk/{i}")).await;
            response.assert_status_ok();
        }
        let response = server.post("/12/place/cookie/2").await;
        response.assert_status_ok();

        for i in 3..5 {
            let response = server.post(&format!("/12/place/milk/{i}")).await;
            response.assert_status_ok();
        }
        let response = server.post("/12/place/cookie/3").await;
        response.assert_status_ok();

        // for i in 4..5 {
        let response = server.post("/12/place/milk/4").await;
        response.assert_status_ok();
        // }
        let response = server.post("/12/place/cookie/4").await;
        response.assert_status_ok();

        response.assert_text(want);
    }

    #[tokio::test]
    async fn test_task_3() {
        let server = test_server();

        let response = server.post("/12/reset").await;
        response.assert_status_ok();
        response.assert_text(EMPTY_BOARD);

        let want = "\
⬜🍪🍪🍪🍪⬜
⬜🥛🍪🍪🥛⬜
⬜🥛🥛🥛🥛⬜
⬜🍪🥛🍪🥛⬜
⬜⬜⬜⬜⬜⬜
";
        let response = server.get("/12/random-board").await;
        // dbg!(response.text());
        response.assert_status_ok();
        response.assert_text(want);

        let want = "\
⬜🍪🥛🍪🍪⬜
⬜🥛🍪🥛🍪⬜
⬜🥛🍪🍪🍪⬜
⬜🍪🥛🥛🥛⬜
⬜⬜⬜⬜⬜⬜
";
        let response = server.get("/12/random-board").await;
        response.assert_status_ok();
        response.assert_text(want);

        let want = "\
⬜🍪🍪🥛🍪⬜
⬜🍪🥛🍪🍪⬜
⬜🥛🍪🍪🥛⬜
⬜🍪🥛🍪🍪⬜
⬜⬜⬜⬜⬜⬜
";
        let response = server.get("/12/random-board").await;
        response.assert_status_ok();
        response.assert_text(want);

        let want = "\
⬜🥛🍪🍪🥛⬜
⬜🥛🍪🍪🍪⬜
⬜🍪🥛🥛🥛⬜
⬜🍪🥛🍪🥛⬜
⬜⬜⬜⬜⬜⬜
";
        let response = server.get("/12/random-board").await;
        response.assert_status_ok();
        response.assert_text(want);

        let want = "\
⬜🥛🥛🥛🍪⬜
⬜🍪🍪🍪🥛⬜
⬜🥛🍪🍪🥛⬜
⬜🍪🥛🥛🍪⬜
⬜⬜⬜⬜⬜⬜
";
        let response = server.get("/12/random-board").await;
        response.assert_status_ok();
        response.assert_text(want);

        let response = server.post("/12/reset").await;
        response.assert_status_ok();
        response.assert_text(EMPTY_BOARD);

        let want = "\
⬜🍪🍪🍪🍪⬜
⬜🥛🍪🍪🥛⬜
⬜🥛🥛🥛🥛⬜
⬜🍪🥛🍪🥛⬜
⬜⬜⬜⬜⬜⬜
";
        let response = server.get("/12/random-board").await;
        response.assert_status_ok();
        response.assert_text(want);
    }
}
