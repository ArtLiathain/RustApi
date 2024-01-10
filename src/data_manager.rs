pub mod data_manager {
    use crate::stock_parsing::stock_parsing::TimeSeries;
    use chrono::NaiveDate;

    pub fn create_stock_date_value_tuple(timeseries: TimeSeries) -> Vec<(NaiveDate, f64, f64)> {
        let mut stock_date_value_tuple: Vec<(NaiveDate, f64, f64)> = Vec::new();
        for (key, value) in timeseries.monthly_time_series {
            stock_date_value_tuple.push((key, value.open, value.close));
        }
        stock_date_value_tuple.sort_by(|a, b| a.0.cmp(&b.0));
        stock_date_value_tuple
    }

    pub fn calculate_daily_returns(
        stock_data: Vec<(NaiveDate, f64, f64)>,
    ) -> Vec<(NaiveDate, f64)> {
        let mut daily_returns: Vec<(NaiveDate, f64)> = Vec::new();
        for (date, open, close) in stock_data {
            daily_returns.push((date, (close / open) * 100.00));
        }
        daily_returns
    }

    pub fn calculate_percentage_of_daily_returns(
        daily_returns_data: Vec<(NaiveDate, f64)>,
    ) -> (f64, f64) {
        let mut positive: f64 = 0.0;
        let mut negative: f64 = 0.0;
        let total: f64 = daily_returns_data.len() as f64;
        for (_date, percentage) in daily_returns_data {
            if percentage > 0.0 {
                positive += 1.0;
                continue;
            }
            negative += 1.0;
        }
        ((positive / total) * 100.00, (negative / total) * 100.00)
    }
}

#[cfg(test)]
mod data_manager_testing {
    use super::data_manager::create_stock_date_value_tuple;
    use crate::stock_parsing::stock_parsing::{serialse_to_timeseries, StockData, TimeSeries};
    use chrono::NaiveDate;
    use core::time;
    use serde_json::Error;
    use std::{collections::HashMap, fs};

    fn setup() -> Result<TimeSeries, Error> {
        let json_body = fs::read_to_string("./src/testing.json").expect("Unable to read file");
        let timeseries = serialse_to_timeseries(json_body);
        timeseries
    }

    #[test]
    fn tuple_generator_has_values() {
        let timeseries = setup().unwrap();
        let tuple = create_stock_date_value_tuple(timeseries);
        assert!(tuple.len() > 0);
    }
}
