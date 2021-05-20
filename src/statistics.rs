use crate::request;
use regex::Regex;
use lazy_static::lazy_static;

use crate::util;
use std::env;
use chrono::prelude::*;

pub async fn send_stats_to_bot(count: i32) {
    let channel_id = env::var("CHANNEL_ID").expect("channel id expected");
    let message_id = env::var("MESSAGE_ID").expect("message id expected");
    let token = env::var("BOT_TOKEN").expect("bot token expected");
    let stat_content = request::get_message(&channel_id, &message_id, &token).await.unwrap().content;
    match get_number_from_text(stat_content.as_str()) {
        Ok(stat_count) => {
            println!("Adding stats together: {x} + {y} = {sum}", x=stat_count, y=count, sum = stat_count + count);
            let new_string = format!("**Attempts**: {count}", count = stat_count + count);
            request::edit_message(&channel_id, &message_id, &token, new_string.as_str()).await.unwrap();
        }
        Err(err) => {
            println!("Couldn't add stats!");
            send_err_to_discord(format!("Couldn't add stats.\n{}", err).as_str()).await.unwrap();
        }
    }
}

pub async fn send_err_to_discord(err: &str) -> util::Result<()>{
    let webhook_id = env::var("WEBHOOK_ID").expect("webhook id expected");
    let message_id = env::var("WEBHOOK_TOKEN").expect("webhook token expected");
    // We have to be careful here, this could be an internet
    // connection issue which means that we should probably
    // fail silently if this function errors out.
    request::send_message(&webhook_id, &message_id,
                          &serde_json::json!({
   "embeds":[
      {
         "title":"Error!",
         "description":"There has been a Rust runtime error!",
         "author":{
            "name":"Rust Bestbuy Scraper",
            "icon_url": "https://rustacean.net/assets/cuddlyferris.png"
         },
          "timestamp": Utc::now().to_rfc3339(),
         "footer": {
            "text": "Errored on "
         },
         "color":15406156,
         "fields":[
            {
               "name":"Error:",
               "value": err,
               "inline":true
            }
         ]
      }
   ]
})
    ).await?;

    Ok(())
}


fn get_number_from_text(text: &str) -> Result<i32, &'static str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\d+").unwrap();
    }

    match RE.captures(text) {
       Some(t) => Ok(t.get(0).unwrap().as_str().parse::<i32>().unwrap()),
        None => Err("Could not parse data.")
    }
}