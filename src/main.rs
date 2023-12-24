use reqwest::{blocking::get, Error, Response, StatusCode};

#[tokio::main]
async fn main() {
    // Make the HTTP request
    let client = reqwest::Client::new();
    let resp = client
        .get("http://httpbin.org/get")
        .header(
            "X-Finnhub-Token",
            "cm3ml71r01qsvtcqsclgcm3ml71r01qsvtcqscm0",
        )
        .send()
        .await;
    match resp {
        Ok(resp) => {
            status(&resp);
            println!("{:#?}", resp);
        }
        _ => return,
    }
    // Call the status method with the response
}

fn status(resp: &Response) {
    if resp.status().is_success() {
        println!("success!");
    } else if resp.status().is_server_error() {
        println!("server error!");
    } else {
        println!("Something else happened. Status: {:?}", resp.status());
    }
}
