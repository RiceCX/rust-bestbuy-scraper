use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscordGetResponse {
    pub id: String,
    #[serde(rename = "type")]
    pub channel_type: i32,
    pub content: String,
    pub channel_id: String,
}

/*
#[derive(Deserialize, Serialize)]
struct AuthorField {
    id: String,
    username: String,
    discriminator: String,
    public_flags: i32,
    bot: Bool
}
*/