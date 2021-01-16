use std::env;
use std::str::FromStr;
use surf;
use serde_json::{Value};
use getopts::Options;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct SlackPostMessageBody {
    token: String,
    channel: String,
    text: String,
    icon_emoji: String,
    username: String
}

async fn post2slack(token: String, text: String) -> surf::Result<()> {
    let url = "https://slack.com/api/chat.postMessage";
    let body = SlackPostMessageBody {
        token: token,
        channel: "#weather".into(),
        text: text,
        icon_emoji: ":rain_cloud:".into(),
        username: "雨ですよBot".into()
    };
    let mime = surf::http::Mime::from_str("application/json; charset=utf-8").unwrap();
    let data = surf::post(url)
        .query(&body)?
        .content_type(mime)
        .recv_string().await?;
    println!("data: {}", data);
    Ok(())
}

#[async_std::main]
async fn main() -> surf::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optopt("i", "appid", "Yahho! JAPAN appid", "APPID");
    opts.optopt("s", "slack-token", "Slack Token", "TOKEN");
    opts.optflag("h", "help", "print this help menu");
    
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    match (matches.opt_str("i"), matches.opt_str("s")) {
        (Some(appid), Some(slack_token)) => {
            let url = format!("https://map.yahooapis.jp/weather/V1/place?output=json&coordinates=136.7247468,35.4064446&appid={}", appid);
            let data = surf::get(url).recv_string().await?;
            let v: Value = serde_json::from_str(&data)?;
            for weather in v["Feature"][0]["Property"]["WeatherList"]["Weather"].as_array().unwrap().iter() {
                if let Some(rainfail) = weather["Rainfall"].as_f64() {
                    if 0.0 < rainfail {
                        let message = format!("date:{}, {}", weather["Date"], weather["Rainfall"]);
                        println!("{}", message);
                        post2slack(slack_token, message).await?;
                        break;
                    }
                }
            }
        },
        _ => println!("No appid"),
    }
    
    Ok(())
}
