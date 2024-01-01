mod stock_parsing;
mod http_handler;


use std::fs::{self};
use crate::stock_parsing::stock_parsing::serialse_to_stockdata;

#[tokio::main]
async fn main() {
    let text = fs::read_to_string("./src/testing.json").expect("Unable to read file");
    print!("{}", text);
    let stockdata = serialse_to_stockdata(text);
    match stockdata {
        Ok(stockdata) => println!("{:?}", stockdata),
        Err(_e) => panic!("NULL"),

    }
}


