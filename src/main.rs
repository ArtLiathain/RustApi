use chrono::{DateTime, Utc};
use reqwest::{Error, Response};
use serde::Deserialize;
#[tokio::main]
async fn main() {
    let resp = make_request("AMD").await;
    let text: Result<String, Errors>;
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

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct TimeSeriesData {
    pub date: DateTime<Utc>,
    pub stock_data: StockData,
}

#[derive(Deserialize)]
struct StockData {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

enum Errors {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),

}

async fn make_request(ticker: &str) -> Result<Response, Error> {
    let client = reqwest::Client::new();
    let resp = client
        .get(format!("https://www.alphavantage.co/query?function=TIME_SERIES_MONTHLY&symbol={}&apikey=PBTAR9NQ563TXSF0", ticker))
        .send()
        .await?;
    Ok(resp)
}

async fn get_response_body(resp: Response) -> Result<String, Error> {
    resp.text().await
}

async fn serialse_to_stockdata(text: String) -> Result<TimeSeriesData, Errors> {
    let data: Result<TimeSeriesData, serde_json::Error> = serde_json::from_str(&text);
    match data {
        Ok(data) => Ok(data),
        Err(e) => Err(Errors::Serde(e)),
    }
} 
