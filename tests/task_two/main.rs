#[cfg(all(test, feature = "task1-9"))]
mod task_two {
    use axum_test::TestServer;
    use itsscb_shuttlings_cch24::router;

    fn test_server() -> TestServer {
        TestServer::new(router()).unwrap()
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
}
