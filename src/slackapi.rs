use std::str::FromStr;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct SlackPostMessagePayload {
    text: String,
    icon_emoji: String,
    username: String
}

pub async fn post2slack(url: impl Into<String>, text: impl Into<String>) -> surf::Result<()> {
    let body = SlackPostMessagePayload {
        text: text.into(),
        icon_emoji: ":rain_cloud:".into(),
        username: "雨ですよBot".into()
    };
    let body_str = serde_json::to_string(&body).unwrap();
    let u = url.into();
    let mime = surf::http::Mime::from_str("application/json").unwrap();
    let data = surf::post(u)
        .body(body_str)
        .content_type(mime)
        .recv_string().await?;
    println!("data: {}", data);
    Ok(())
}
