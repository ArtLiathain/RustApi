pub mod data_manager {
    use chrono::NaiveDate;
    use crate::stock_parsing::stock_parsing::TimeSeries;

    pub fn create_stock_date_value_tuple(timeseries: TimeSeries) -> Vec<(NaiveDate, f64)> {
        let mut stock_date_value_tuple: Vec<(NaiveDate, f64)> = Vec::new();
        for (key, value) in timeseries.monthly_time_series {
            let mean = (value.open + value.high + value.low + value.close)/4.0;
            stock_date_value_tuple.push((key, mean));
        }
        stock_date_value_tuple.sort_by(|a, b| a.0.cmp(&b.0));
        stock_date_value_tuple
    }
}

#[cfg(test)]
mod data_manager_testing {
    use core::time;
    use std::{collections::HashMap, fs};
    use chrono::NaiveDate;
    use serde_json::Error;
    use crate::stock_parsing::stock_parsing::{TimeSeries, StockData, serialse_to_timeseries};
    use super::data_manager::create_stock_date_value_tuple;

    fn setup() ->  Result<TimeSeries, Error> {
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