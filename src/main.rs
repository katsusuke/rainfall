use std::env;
use std::str::FromStr;
use surf;
use getopts::Options;
use serde::{Deserialize, Serialize};

mod yahooapi;

#[derive(Deserialize, Serialize)]
struct SlackPostMessagePayload {
    token: String,
    channel: String,
    text: String,
    icon_emoji: String,
    username: String
}

async fn post2slack(token: String, text: String) -> surf::Result<()> {
    let url = "https://slack.com/api/chat.postMessage";
    let body = SlackPostMessagePayload {
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

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

async fn check_rainfall(watch: bool, appid: String, slack_token: String, coordinates: String) -> surf::Result<()> {
    if let Some(w) = yahooapi::find_rainfail(appid, coordinates).await? {
        let message = format!("date:{}, rainfail: {}", w.date.to_string(), w.rainfail);
        println!("{}", message);
        post2slack(slack_token, message).await?;
    }
    Ok(())
}

#[async_std::main]
async fn main() -> surf::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optopt("i", "appid", "Yahho! JAPAN appid(Required)", "APPID");
    opts.optopt("s", "slack-token", "Slack Token(Required)", "TOKEN");
    opts.optopt("c", "coordinates", "latitude,longitude(Required)", "LATITUDE,LONGITUDE");
    opts.optflag("w", "watch", "Service mode");
    opts.optflag("h", "help", "print this help menu");
    
    let m = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    
    if m.opt_present("h") {
        let program = args[0].clone();
        print_usage(&program, opts);
        return Ok(());
    }

    match (m.opt_str("i"), m.opt_str("s"), m.opt_str("c")) {
        (Some(appid), Some(slack_token), Some(coordinates)) => {
            check_rainfall(m.opt_present("w"), appid, slack_token, coordinates).await?
        },
        _ => println!("No appid"),
    }
    
    Ok(())
}
