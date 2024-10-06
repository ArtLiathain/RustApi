pub mod http_handler {
    use crate::{
        data_manager::data_manager::*,
        stock_parsing::stock_parsing::{serialse_to_timeseries, TimeSeries},
    };
    use actix_web::{
        cookie::time::Time, get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
    };
    use chrono::NaiveDate;
    use reqwest::Response;

    use serde::{Deserialize, Serialize};

    #[derive(Serialize)]
    struct ApiResponse {
        stock_data: Vec<(NaiveDate, f64, f64)>, // Assuming a tuple of (date, value)
        daily_returns: Vec<(NaiveDate, f64)>,
        annualised_returns: f64,
        omega_ratio: f64,
        daily_returns_percentage: (f64, f64),
        rolling_returns: Vec<(NaiveDate, f64)>,
    }

    #[derive(Debug, Deserialize)]
    pub struct Params {
        ticker: String,
    }

    #[get("/")]
    pub async fn hello() -> impl Responder {
        HttpResponse::Ok().body("Hello world!")
    }

    #[get("/getStats")]
    pub async fn echo(req: HttpRequest) -> impl Responder {
        let mut time_series: TimeSeries;
        let params =
            web::Query::<Params>::from_query(req.query_string()).unwrap_or(web::Query(Params {
                ticker: String::from(""),
            }));

        match make_request(format!("https://www.alphavantage.co/query?function=TIME_SERIES_MONTHLY&symbol={}&apikey=PBTAR9NQ563TXSF0", params.ticker)).await
        {
            Ok(data) => match data.text().await {
                Ok(data) => {match serialse_to_timeseries(data) {
                    Ok(data) => {time_series = data},
                    Err(_) => {
                        return HttpResponse::BadRequest().body("Error parsing Stock Data")}
                }
            },
            Err(_) => return HttpResponse::BadRequest().body("Error getting Stock Data") 
            },
            Err(_) => return HttpResponse::BadRequest().body("Error getting Stock Data") 
        }
        let stock_data = create_stock_date_value_tuple(time_series);
        let daily_returns = calculate_daily_returns(&stock_data);
        let annualised_returns = calculate_annualised_returns(&stock_data);
        let omega_ratio_value = omega_ratio(&daily_returns);
        let daily_returns_percentage = calculate_percentage_of_daily_returns(&daily_returns);
        let rolling_returns = monthly_year_rolling_returns(&stock_data);

        let response = ApiResponse {
            stock_data,
            daily_returns,
            annualised_returns,
            omega_ratio: omega_ratio_value,
            daily_returns_percentage,
            rolling_returns: rolling_returns,
        };

        HttpResponse::Ok().json(response)
    }

    pub async fn manual_hello() -> impl Responder {
        HttpResponse::Ok().body("Hey there!")
    }

    async fn make_request(url: String) -> Result<Response, reqwest::Error> {
        let client = reqwest::Client::new();
        let resp = client.get(url).send().await?;
        Ok(resp)
    }

    // #[cfg(test)]
    // mod http_handler_tests {
    //     use super::make_request;
    //     use mockito::ServerGuard;

    //     fn setup_server() -> (ServerGuard, mockito::Mock) {
    //         let mut server = mockito::Server::new();

    //         let mock = server
    //             .mock("GET", "/AMD")
    //             .with_status(200)
    //             .with_header("content-type", "text/plain")
    //             .with_body("world")
    //             .create();

    //         (server, mock)
    //     }

    //     #[actix_web::test]
    //     async fn test_api_call() {
    //         let (server, mock) = setup_server();
    //         let url = format!("{}/{}", server.url(), "AMD");
    //         let _ = make_request(url).await;
    //         mock.assert();
    //     }

    //     #[actix_web::test]
    //     async fn response_body_works() {
    //         let (server, _mock) = setup_server();
    //         let url = format!("{}/{}", server.url(), "AMD");
    //         let response = make_request(url).await;
    //         match response {
    //             Ok(response) => {
    //                 let body = response.text().await;
    //                 match body {
    //                     Ok(body) => assert_eq!(body, "world"),
    //                     Err(_) => assert!(false),
    //                 }
    //             }
    //             Err(_) => assert!(false),
    //         }
    //     }
    // }
}
