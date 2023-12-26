use reqwest::{Error, Response};

#[tokio::main]
async fn main() {
    let resp = make_request().await;
    let text : Result<String, Error>;
    match resp {
        Ok(resp) => text = get_response_body(resp).await,
        _ => panic!("EMPTY"),
    };
    match text {
        Ok(text) => {
            print!("{}", text);
        }
        _ => panic!("Empty"),
    }
}

async fn make_request() -> Result<Response, Error> {
    let client = reqwest::Client::new();
    let resp = client
        .get("https://finnhub.io/api/v1/search?q=apple")
        .header(
            "X-Finnhub-Token",
            "cm3ml71r01qsvtcqsclgcm3ml71r01qsvtcqscm0",
        )
        .send()
        .await?;
    Ok(resp)
}

async fn get_response_body(resp : Response) -> Result<String, Error> {
    resp.text().await
}
