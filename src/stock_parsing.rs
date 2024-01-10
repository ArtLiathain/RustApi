pub mod stock_parsing {
    use chrono::NaiveDate;
    use serde::Deserialize;
    use serde_json::Error;
    use serde_this_or_that::as_f64;
    use std::collections::HashMap;

    #[derive(Deserialize, Debug)]
    pub struct TimeSeries {
        #[serde(rename = "Monthly Time Series")]
        pub monthly_time_series: HashMap<NaiveDate, StockData>,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct StockData {
        #[serde(rename = "1. open")]
        #[serde(deserialize_with = "as_f64")]
        pub open: f64,
        #[serde(rename = "2. high")]
        #[serde(deserialize_with = "as_f64")]
        pub high: f64,
        #[serde(rename = "3. low")]
        #[serde(deserialize_with = "as_f64")]
        pub low: f64,
        #[serde(rename = "4. close")]
        #[serde(deserialize_with = "as_f64")]
        pub close: f64,
        #[serde(rename = "5. volume")]
        #[serde(deserialize_with = "as_f64")]
        pub volume: f64,
    }

    pub fn serialse_to_timeseries(text: String) -> Result<TimeSeries, Error> {
        let data: Result<TimeSeries, serde_json::Error> = serde_json::from_str(&text);
        match data {
            Ok(data) => Ok(data),
            Err(e) => Err(e),
        }
    }

    #[cfg(test)]
    mod stock_parsing_tests {
        use super::serialse_to_timeseries;
        use std::fs;

        fn setup() -> String {
            fs::read_to_string("./src/testing.json").expect("Unable to read file")
        }

        #[test]
        fn test_sts_incorrect_input() {
            let timeseries = serialse_to_timeseries("Empty".to_string());
            match timeseries {
                Ok(_) => assert!(false),
                Err(_) => assert!(true),
            }
        }

        #[test]
        fn test_sts_create_object() {
            let json_body = setup();
            let timeseries = serialse_to_timeseries(json_body);
            match timeseries {
                Ok(timeseries) => assert!(timeseries.monthly_time_series.keys().len() > 0),
                Err(_e) => assert!(false),
            }
        }

        #[test]
        fn test_sts_input_is_indexable() {
            let json_body = setup();
            let timeseries = serialse_to_timeseries(json_body);
            match timeseries {
                Ok(timeseries) => {
                    for (_key, stockvalue) in &timeseries.monthly_time_series {
                        assert!(!stockvalue.open.is_nan())
                    }
                }
                Err(_e) => assert!(false),
            }
        }
    }
}
