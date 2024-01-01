pub mod http_handler {
    use reqwest::Response;

    async fn make_request(url: String) -> Result<Response, reqwest::Error> {
        let client = reqwest::Client::new();
        let resp = client.get(url).send().await?;
        Ok(resp)
    }

    #[cfg(test)]
    mod http_handler_tests {
        use super::make_request;
        use mockito::ServerGuard;

        fn setup_server() -> (ServerGuard, mockito::Mock) {
            let mut server = mockito::Server::new();

            let mock = server
                .mock("GET", "/AMD")
                .with_status(200)
                .with_header("content-type", "text/plain")
                .with_body("world")
                .create();

            (server,mock)
        }

        #[tokio::test]
        async fn test_api_call() {
            let (server, mock) = setup_server();
            let url = format!("{}/{}", server.url(), "AMD");
            let _ = make_request(url).await;
            mock.assert();
        }

        #[tokio::test]
        async fn response_body_works() {
            let (server, _mock) = setup_server();
            let url = format!("{}/{}", server.url(), "AMD");
            let response = make_request(url).await;
            match response {
                Ok(response) => {
                    let body = response.text().await;
                    match body {
                        Ok(body) => assert_eq!(body, "world"),
                        Err(_) => assert!(false),
                    }
                }
                Err(_) => assert!(false),
            }
        }
    }
}
