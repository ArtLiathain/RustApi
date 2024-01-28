pub mod data_manager {
    use crate::stock_parsing::stock_parsing::TimeSeries;
    use chrono::{offset, Datelike, NaiveDate};

    pub fn create_stock_date_value_tuple(timeseries: TimeSeries) -> Vec<(NaiveDate, f64, f64)> {
        let mut stock_date_value_tuple: Vec<(NaiveDate, f64, f64)> = Vec::new();
        for (key, value) in timeseries.monthly_time_series {
            stock_date_value_tuple.push((key, value.open, value.close));
        }
        stock_date_value_tuple.sort_by(|a, b| b.0.cmp(&a.0));
        stock_date_value_tuple
    }

    pub fn calculate_daily_returns(
        stock_data: Vec<(NaiveDate, f64, f64)>,
    ) -> Vec<(NaiveDate, f64)> {
        let mut daily_returns: Vec<(NaiveDate, f64)> = Vec::new();
        for (date, open, close) in stock_data {
            daily_returns.push((date, (((close / open) - 1.00) * 100000.00).round() / 1000.00));
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
            } else if percentage < 0.0 {
                negative += 1.0;
                continue;
            }
        }
        (
            ((positive / total) * 100.00).round(),
            ((negative / total) * 100.00).round(),
        )
    }

    pub fn calculate_annualised_returns(stock_data: Vec<(NaiveDate, f64, f64)>) -> f64 {
        let overall_return: f64 = (stock_data.last().unwrap().2 - stock_data.first().unwrap().1)
            / stock_data.first().unwrap().1;
        let number_of_years = (stock_data.last().unwrap().0 - stock_data.first().unwrap().0)
            .num_days() as f64
            / 365.00;
        (1.00 + overall_return).powf(1.00 / number_of_years) - 1.00
    }

    pub fn omega_ratio(daily_returns_data: Vec<(NaiveDate, f64)>) -> f64 {
        let mut positive: f64 = 0.0;
        let mut negative: f64 = 0.0;
        for (_date, percentage) in daily_returns_data {
            if percentage > 0.0 {
                positive += percentage;
                continue;
            } else if percentage < 0.0 {
                negative += percentage;
                continue;
            }
        }
        
        ((positive / negative.abs()) * 1000.00).round() / 1000.00
    }

    pub fn rolling_returns(stock_data: Vec<(NaiveDate, f64, f64)>) -> Vec<(NaiveDate, f64)>{
        let mut rolling_returns: Vec<(NaiveDate, f64)> = Vec::new();
        let mut started_window = false;
        let mut offset = 0;
        println!("Stockdata length is {}", stock_data.len());

        for i in 0..stock_data.len(){
            println!("i is {}", i);
            if started_window == true{
                rolling_returns.push((stock_data[i].0, (((stock_data[i].1 / stock_data[i].1) - 1.00) * 100000.00).round() / 1000.00));
                continue;
            }
            for j in 0..stock_data.len(){
                println!("j is {}", j);

                 if stock_data[i].0.month() == stock_data[j].0.month() && stock_data[i].0.year() == stock_data[j].0.year() + 1{
                     println!("i and j are {} and {}", i, j);
                     rolling_returns.push((stock_data[i].0, (((stock_data[i].1 / stock_data[j].1) - 1.00) * 100000.00).round() / 1000.00));
                     started_window = true;
                     break;
                 }
                 if stock_data[j].0 > stock_data[i].0 {
                     break;
                 }
                
            }
        }
        rolling_returns.push((stock_data.last().unwrap().0, 2.00));
        rolling_returns
    }
}

#[cfg(test)]
pub mod data_manager_testing {
    use super::data_manager::create_stock_date_value_tuple;
    use crate::stock_parsing::stock_parsing::{serialse_to_timeseries, TimeSeries};
    use serde_json::Error;

    fn setup() -> Result<TimeSeries, Error> {
        let json_body = r#"
            {"Monthly Time Series": {
                "2024-10-29": {
                "1. open": "158.4100",
                "2. high": "166.3400",
                "3. low": "158.0000",
                "4. close": "163.5500",
                "5. volume": "87358302"
                },
                "2023-10-30": {
                "1. open": "145.0000",
                "2. high": "158.6000",
                "3. low": "144.4500",
                "4. close": "158.5600",
                "5. volume": "78460252"
                },
                "2022-10-31": {
                "1. open": "140.0400",
                "2. high": "144.7600",
                "3. low": "135.8700",
                "4. close": "138.6400",
                "5. volume": "94386980"
                }
            }}"#;
        let timeseries = serialse_to_timeseries(json_body.to_string());
        timeseries
    }

    #[test]
    fn tuple_generator_has_values() {
        let timeseries = setup().unwrap();
        let tuple = create_stock_date_value_tuple(timeseries);
        assert!(tuple.len() > 0);
    }

    #[test]
    fn is_daily_returns_correct() {
        let timeseries = setup().unwrap();
        let tuple = create_stock_date_value_tuple(timeseries);
        let daily_returns = super::data_manager::calculate_daily_returns(tuple);
        print!("{:?}", daily_returns);
        assert!(daily_returns[0].1 == 3.245);
    }

    #[test]
    fn is_percentage_daily_returns_correct() {
        let timeseries = setup().unwrap();
        let tuple = create_stock_date_value_tuple(timeseries);
        let daily_returns = super::data_manager::calculate_daily_returns(tuple);
        let percentage = super::data_manager::calculate_percentage_of_daily_returns(daily_returns);
        assert!(percentage.1 == 33.00);
        assert!(percentage.0 == 67.00);
    }

    #[test]
    fn omega_ratio_test() {
        let timeseries = setup().unwrap();
        let tuple = create_stock_date_value_tuple(timeseries);
        let daily_returns = super::data_manager::calculate_daily_returns(tuple);
        let omega_ratio = super::data_manager::omega_ratio(daily_returns);
        assert!(omega_ratio == 12.597);
    }

    #[test]
    fn rolling_returns_test() {
        let timeseries = setup().unwrap();
        let tuple = create_stock_date_value_tuple(timeseries);
        print!("{:?}", tuple.len());
        let rolling_returns = super::data_manager::rolling_returns(tuple);
        println!("{:?}", rolling_returns.len());
        println!("Rolling Returns");
        assert!(rolling_returns.len() == 1);
    }
}
