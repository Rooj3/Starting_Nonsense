use std::{collections::HashMap, future::IntoFuture};
use serde::{Deserialize, Serialize};
use json::{Error, JsonValue};
use reqwest::{Client, Response};

struct AVTotal {
    meta_data : AVMeta,
    time_series: AVTime,
}
struct AVMeta {
    information: String,
    symbol: String,
    last_refreshed: String,
    interval: String,
    output_size: String,
    time_zone: String,
}
struct AVTime {
    series: JsonValue
}

struct AlphaVantageRequest {
    apikey: String,
    symbol: String,
    interval: Interval,
    // optional_args: OptionalFields
}

impl AlphaVantageRequest {
    fn new(apikey: String, symbol: String, interval: Interval) -> AlphaVantageRequest {
        AlphaVantageRequest {
            apikey,
            symbol,
            interval,
            // optional_args
        }
    }

    async fn get_data(&self) -> JsonValue {
        let x = AlphaVantageRequest::make_get(self.generate_url()).await;
        x
    }

    fn generate_url(&self) -> String {
        
        let url = String::from("https://www.alphavantage.co/query?");

        //always INTRADAY:
        let function = String::from("&function=TIME_SERIES_INTRADAY");

        let mut interval = String::from("&interval=");
        match self.interval {
            Interval::Min1 => interval.push_str("1min"),
            Interval::Min5 => interval.push_str("5min"),
            Interval::Min15 =>interval.push_str("15min"),
            Interval::Min30 =>interval.push_str("30min"),
            Interval::Min60 =>interval.push_str("60min"),
        }

        let apikey = format!("&apikey={0}", self.apikey);
        let symbol = format!("&symbol={0}", self.symbol);

        let url = format!("{url}{function}{interval}{symbol}{apikey}");
        url
    }

    async fn make_get(url: String) -> JsonValue {
        let client = Client::new();
        let response = client.get(url).send().await.unwrap().text().await.unwrap();
        let x = json::parse(&response).unwrap();
        x
        
    }

}

struct OptionalFields {
    fields: HashMap<String,String>
}


enum Interval {
    Min1, 
    Min5,
    Min15,
    Min30, 
    Min60
}



#[tokio::main]
async fn main() {
    let api_key = String::from("LSWD3BOZ5HG1ZSJV");
    let connect = AlphaVantageRequest::new(api_key, String::from("IBM"), Interval::Min1);
    let json_return = connect.get_data().await;
    println!("{}", json_return["Meta Data"])
}
