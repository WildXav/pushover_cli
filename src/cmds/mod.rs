use reqwest::Client;
use serde_json::{Map, Number, Value};

use crate::structs::Config;

pub(crate) async fn send_notif(
    client: &Client,
    config: &Config,
    title: String,
    msg: String
) -> Result<(), anyhow::Error> {
    let mut req_body = Map::new();
    req_body.insert(String::from("token"), Value::String(config.app_token.clone()));
    req_body.insert(String::from("user"), Value::String(config.user_token.clone()));
    req_body.insert(String::from("title"), Value::String(title));
    req_body.insert(String::from("message"), Value::String(msg));
    req_body.insert(String::from("html"), Value::Number(Number::from(1)));

    let res = client
        .post("https://api.pushover.net/1/messages.json")
        .json(&req_body)
        .send()
        .await?;

    res.error_for_status()?;

    Ok(())
}