use reqwest;
use reqwest::header::{ACCEPT_LANGUAGE, AUTHORIZATION, CONTENT_TYPE, HeaderMap, USER_AGENT};

use crate::webhook;
use crate::util;
use serde_json::Value;




pub async fn send_request() -> util::Result<String> {
    let mut headers: HeaderMap = HeaderMap::new();
    set_headers(&mut headers);

    let req_client = reqwest::Client::new();
    let res = req_client.get(util::URL).headers(headers).send()
        .await?
        .text()
        .await?;

    Ok(res)
}
pub async fn edit_message(channel_id: &str, message_id: &str, token: &str, new_message: &str) -> util::Result<()> {
    let mut discord_headers: HeaderMap = HeaderMap::new();
    set_discord_headers(&mut discord_headers, token);

    let req_client = reqwest::Client::new();
    let res = req_client.patch(format!("{base_url}/channels/{channel_id}/messages/{message_id}", base_url = util::DISCORD_BASEURL, channel_id = channel_id, message_id = message_id)).headers(discord_headers)
        .json(&serde_json::json!({
            "content": new_message
        }))
        .send()
        .await?;

    if res.status() == 200 || res.status() == 204 {
        Ok(())
    } else {
        Err("Could not edit message".into())
    }
}
pub async fn send_message(webhook_id: &str, webhook_token: &str, body: &Value) -> util::Result<()> {
    let mut discord_headers: HeaderMap = HeaderMap::new();
    discord_headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    discord_headers.insert(USER_AGENT, "RiceCX Daemon Service/1.0".parse().unwrap());
    let req_client = reqwest::Client::new();
    let res = req_client.post(
        format!("{base_url}/webhooks/{webhook_id}/{token}",
                base_url = util::DISCORD_BASEURL,
                webhook_id = webhook_id,
                token = webhook_token
        ))
        .headers(discord_headers)
        .json(body)
        .send()
        .await?;

    if res.status() == 200 || res.status() == 204 {
        Ok(())
    } else {
        Err(format!("Could not send webhook message. Status returned: {}", res.status()).into())
    }
}

pub async fn get_message(channel_id: &str, message_id: &str, token: &str) -> util::Result<webhook::DiscordGetResponse> {
    let mut discord_headers: HeaderMap = HeaderMap::new();
    set_discord_headers(&mut discord_headers, token);
    let req_client = reqwest::Client::new();
    let res: webhook::DiscordGetResponse = req_client.get(
        format!("{base_url}/channels/{channel_id}/messages/{message_id}",
                base_url = util::DISCORD_BASEURL,
                channel_id = channel_id,
                message_id = message_id
        ))
        .headers(discord_headers).send()
        .await?
        .json()
        .await?;

    Ok(res)
}

fn set_discord_headers(headers: &mut HeaderMap, token: &str) {
    headers.insert(AUTHORIZATION, format!("Bot {token}", token = token).parse().unwrap());
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert(USER_AGENT, "RiceCX Daemon Service/1.0".parse().unwrap());
}
fn set_headers(headers: &mut HeaderMap) {
    headers.insert(USER_AGENT, (util::UA).parse().unwrap());
    headers.insert(ACCEPT_LANGUAGE, "en-US;en;q=0.9".parse().unwrap());
    //  headers.insert(ACCEPT_ENCODING, "gzip,deflate,br".parse().unwrap());
    headers.insert(CONTENT_TYPE, "text/html".parse().unwrap());
}
