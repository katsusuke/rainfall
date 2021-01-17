use std::env;
use std::time::Duration;
use getopts::Options;
use async_std::task;
use chrono::prelude::*;

mod yahooapi;
mod slackapi;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn message_for_rainfail(w: &yahooapi::Weather) -> String {
    let message = format!("date:{}, rainfail: {}", w.date.to_string(), w.rainfail);
    println!("{}", message);
    return message;
}

async fn check_rainfall(appid: &str, coordinates: &str, slack_token: &str) -> Duration {
    let default_wait = Duration::from_secs(60 * 10);
    if let Ok(Some(w)) = yahooapi::find_rainfail(appid, coordinates).await {
        let message = message_for_rainfail(&w);
        let _ = slackapi::post2slack(slack_token, message).await;
        let now = Local::now().naive_utc();
        let duration = (w.date + chrono::Duration::seconds(60 * 60 * 6)) - now;
        duration.to_std().unwrap_or(default_wait)
    } else {
        default_wait
    }
}

async fn watch(appid: &str, coordinates: &str, slack_token: &str) -> Duration {
    loop {
        let wait = check_rainfall(appid, coordinates, slack_token).await;
        println!("sleep: {}secs", wait.as_secs());
        task::sleep(wait).await;
    }
}

#[async_std::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optopt("i", "appid", "Yahho! JAPAN appid(Required)", "APPID");
    opts.optopt("c", "coordinates", "latitude,longitude(Required)", "LATITUDE,LONGITUDE");
    opts.optopt("s", "slack-token", "Slack Token(Required)", "TOKEN");
    opts.optflag("w", "watch", "Service mode");
    opts.optflag("h", "help", "print this help menu");
    
    let m = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    
    if m.opt_present("h") {
        let program = args[0].clone();
        print_usage(&program, opts);
        return;
    }

    match (m.opt_str("i"), m.opt_str("c"), m.opt_str("s")) {
        (Some(appid), Some(slack_token), Some(coordinates)) => {
            if m.opt_present("w") {
                watch(&appid, &slack_token, &coordinates).await;
            } else {
                check_rainfall(&appid, &slack_token, &coordinates).await;
            }
        },
        _ => println!("No appid"),
    }
}
