use std::env;
use surf;
use serde_json::{Value};
use getopts::Options;

#[async_std::main]
async fn main() -> surf::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optopt("i", "appid", "Yahho! JAPAN appid", "APPID");
    opts.optflag("h", "help", "print this help menu");
    
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    match matches.opt_str("i") {
        Some(appid) => {
            let url = format!("https://map.yahooapis.jp/weather/V1/place?output=json&coordinates=136.7247468,35.4064446&appid={}", appid);
            let data = surf::get(url).recv_string().await?;
            let v: Value = serde_json::from_str(&data)?;
            for weather in v["Feature"][0]["Property"]["WeatherList"]["Weather"].as_array().unwrap().iter() {
                if let Some(rainfail) = weather["Rainfall"].as_f64() {
                    if 0.0 < rainfail {
                        println!("date:{}, {}", weather["Date"], weather["Rainfall"]);
                        break;
                    }
                }
            }
        },
        None => println!("No appid")
    }
    
    Ok(())
}
