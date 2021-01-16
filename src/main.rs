use std::env;
use surf;
use getopts::Options;

mod yahooapi;
mod slackapi;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

async fn check_rainfall(watch: bool, appid: String, slack_token: String, coordinates: String) -> surf::Result<()> {
    if let Some(w) = yahooapi::find_rainfail(appid, coordinates).await? {
        let message = format!("date:{}, rainfail: {}", w.date.to_string(), w.rainfail);
        println!("{}", message);
        slackapi::post2slack(slack_token, message).await?;
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
