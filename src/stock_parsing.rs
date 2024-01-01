pub mod stock_parsing {
    use serde::Deserialize;
    use serde_json::Error;
    use std::collections::HashMap;



    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct StockData {
        #[serde(rename = "1. open")]
        pub open: String,
        #[serde(rename = "2. high")]
        pub high: String,
        #[serde(rename = "3. low")]
        pub low: String,
        #[serde(rename = "4. close")]
        pub close: String,
        #[serde(rename = "5. volume")]
        pub volume: Option<String>,
    }

    pub fn serialse_to_stockdata(text: String) -> Result<HashMap<String, StockData>, Error> {
        let data: Result<HashMap<String, StockData>, serde_json::Error> =
            serde_json::from_str(&text);
        match data {
            Ok(data) => Ok(data),
            Err(e) => Err(e),
        }
    }



    
    #[cfg(test)]
    mod stock_parsing_tests {
        use super::serialse_to_stockdata;
        use chrono::NaiveDate;
        use std::fs;

        fn setup() -> String {
            fs::read_to_string("./src/testing.json").expect("Unable to read file")
        }

        #[test]
        fn test_sts_incorrect_input() {
            let stockdata = serialse_to_stockdata("Empty".to_string());
            match stockdata {
                Ok(_) => assert!(false),
                Err(_) => assert!(true),
            }
        }

        #[test]
        fn test_sts_create_object() {
            let json_body = setup();
            let stockdata = serialse_to_stockdata(json_body);
            match stockdata {
                Ok(stockdata) => assert!(stockdata.keys().len() > 0),
                Err(_e) => assert!(false),
            }
        }

        #[test]
        fn test_sts_input_is_indexable() {
            let json_body = setup();
            let stockdata = serialse_to_stockdata(json_body);
            match stockdata {
                Ok(stockdata) => {
                    for (_key, stockvalue) in &stockdata {
                        assert!(!stockvalue.open.is_empty())
                    }
                }
                Err(_e) => assert!(false),
            }
        }

        #[test]
        fn test_sts_keys_are_dates() {
            let json_body = setup();
            let stockdata = serialse_to_stockdata(json_body);
            match stockdata {
                Ok(stockdata) => {
                    for (key, _stockvalue) in &stockdata {
                        assert!(NaiveDate::parse_from_str(key, "%Y-%m-%d").is_ok());
                    }
                }
                Err(_e) => assert!(false),
            }
        }
    }
}
