use surf;
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

pub struct Weather {
    pub is_forecast: bool,
    pub date: NaiveDateTime,
    pub rainfail: f64
}

pub async fn find_rainfail(appid: String, coordinates: String) -> Result<Option<Weather>, surf::Error> {
    let v = getweather(coordinates, appid).await?;
    for weather in v["Feature"][0]["Property"]["WeatherList"]["Weather"].as_array().unwrap().iter() {
        if let (Some(rainfail), Some(date_str)) = (weather["Rainfall"].as_f64(), weather["Date"].as_str()) {
            if let Ok(date) = parse_yahoo_date(date_str) {
                if 0.0 < rainfail {
                    let w = Weather {
                        is_forecast: weather["Type"] == "forecast",
                        date: date,
                        rainfail: rainfail,
                    };
                    return Ok(Some(w));
                }
            }
        }
    }
    Ok(None)
}
