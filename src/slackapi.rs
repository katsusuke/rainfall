use std::str::FromStr;
use serde::{Deserialize, Serialize};

// #[derive(Deserialize, Serialize)]
// struct SlackIncomingWebhookPayload {
//     text: String
// }

// pub async fn post_webhook(url: impl Into<String>, text: impl Into<String>) -> surf::Result<String> {
//     let body = SlackIncomingWebhookPayload {
//         text: text.into()
//     };
//     let body_str = serde_json::to_string(&body).unwrap();
//     let u = url.into();
//     let mime = surf::http::Mime::from_str("application/json").unwrap();
//     let data = surf::post(u)
//         .body(body_str)
//         .content_type(mime)
//         .recv_string().await?;
//     Ok(data)
// }

#[derive(Deserialize, Serialize)]
struct SlackPostMessagePayload {
    channel: String,
    icon_emoji: String,
    text: String,
}

pub async fn post_message(token: impl Into<String>, channel: impl Into<String>, text: impl Into<String>) -> surf::Result<String> {
    let body = SlackPostMessagePayload {
        channel: channel.into(),
        text: text.into(),
        icon_emoji: ":rain_cloud:".into(),
    };
    let body_str = serde_json::to_string(&body).unwrap();
    let mime = surf::http::Mime::from_str("application/json").unwrap();
    let authorization = format!("Bearer {}", token.into());
    let data = surf::post("https://slack.com/api/chat.postMessage")
        .body(body_str)
        .content_type(mime)
        .header("Authorization", authorization)
        .recv_string().await?;
    Ok(data)
}
