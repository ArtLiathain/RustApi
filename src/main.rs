use std::{fs::{self, File}, collections::HashMap};

use chrono::{DateTime, Utc};
use reqwest::{Error, Response};
use serde::Deserialize;
#[tokio::main]
async fn main() {
    // let respp = make_request("AMD").await;
    // let text: Result<String, reqwest::Error>;
    // match resp {
    //     Ok(resp) => text = get_response_body(resp).await,
    //     _ => panic!("EMPTY"),
    // };
    // let temp : Result<TimeSeriesData, Errors>;
    // match text {
    //     Ok(text) => {
    //         temp = serialse_to_stockdata(text).await;
    //     }
    //     _ => panic!("Empty"),
    // }
    let text = fs::read_to_string("./src/testing.json").expect("Unable to read file");
    print!("{}", text);
    let temp = serialse_to_stockdata(text).await;
    match temp {
        Ok(temp) => println!("{:?}", temp.get("2023-12-29").unwrap().high),
        Err(e) => panic!("NULL"),

    }
}

#[derive(Debug,Deserialize)]
#[serde(rename_all = "PascalCase")]


enum Errors {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),

}

async fn make_request(ticker: &str) -> Result<Response, reqwest::Error> {
    let client = reqwest::Client::new();
    let resp = client
        .get(format!("https://www.alphavantage.co/query?function=TIME_SERIES_MONTHLY&symbol={}&apikey=PBTAR9NQ563TXSF0", ticker))
        .send()
        .await?;
    Ok(resp)
}

async fn get_response_body(resp: Response) -> Result<String, reqwest::Error> {
    resp.text().await
}


struct TimeSeriesData {
    pub date: DateTime<Utc>,
    pub stock_data: StockData,
}

#[derive(Debug,Deserialize)]
struct StockData {
    #[serde(rename = "1. open")]
    open: String,
    #[serde(rename = "2. high")]
    high: String,
    #[serde(rename = "3. low")]
    low: String,
    #[serde(rename = "4. close")]
    close: String,
    #[serde(rename = "5. volume")]
    volume: String,
}

async fn serialse_to_stockdata(text: String) -> Result<HashMap<String, StockData>, Errors> {
    let data: Result<HashMap<String, StockData>, serde_json::Error> = serde_json::from_str(&text);
    match data {
        Ok(data) => Ok(data),
        Err(e) => Err(Errors::Serde(e)),
    }
} 
