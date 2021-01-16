use chrono::prelude::*;
use serde_json::{Value};

pub fn parse_yahoo_date(string: &str) -> chrono::ParseResult<NaiveDateTime> {
    NaiveDateTime::parse_from_str(string, "%Y%m%d%H%M")
}

pub async fn getweather(coordinates: String, appid: String) -> surf::Result<Value> {
    let url = format!("https://map.yahooapis.jp/weather/V1/place?output=json&coordinates={}&appid={}", coordinates, appid);
    let data = surf::get(url).recv_string().await?;
    Ok(serde_json::from_str(&data)?)
}
