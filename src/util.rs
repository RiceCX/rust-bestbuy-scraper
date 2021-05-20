use crate::request;
use std::env;


pub const URL: &str = "https://www.bestbuy.com/site/nvidia-geforce-rtx-3080-10gb-gddr6x-pci-express-4-0-graphics-card-titanium-and-black/6429440.p?skuId=6429440";
// pub const URL: &str = "https://example.com";
pub const UA: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/5321 (KHTML, like Gecko) Chrome/36.0.832.0 Mobile Safari/5321";

pub const DISCORD_BASEURL: &str = "https://discord.com/api";
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub async fn send_success_message() -> Result<()>{
    let webhook_id = env::var("WEBHOOK_ID").expect("webhook id expected");
    let message_id = env::var("WEBHOOK_TOKEN").expect("webhook token expected");
    // We have to be careful here, this could be an internet
    // connection issue which means that we should probably
    // fail silently if this function errors out.

    request::send_message(&webhook_id, &message_id, &serde_json::json!({"content": format!("@everyone THERE IS AN RTX 3080 AVAILABLE [Buy here]({})", URL)})).await?;

    Ok(())
}