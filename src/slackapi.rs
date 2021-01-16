use std::str::FromStr;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct SlackPostMessagePayload {
    token: String,
    channel: String,
    text: String,
    icon_emoji: String,
    username: String
}

pub async fn post2slack(token: String, text: String) -> surf::Result<()> {
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
