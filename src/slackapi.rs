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

pub async fn post2slack(token: impl Into<String>, channel: impl Into<String>, text: impl Into<String>) -> surf::Result<()> {
    let url = "https://slack.com/api/chat.postMessage";
    let body = SlackPostMessagePayload {
        token: token.into(),
        channel: channel.into(),
        text: text.into(),
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
