use std::sync::Arc;
use std::sync::atomic::{AtomicI32, Ordering};
use std::thread::sleep;
use std::time::Duration;

use chrono::prelude::*;
use chrono::Utc;
use tokio::{runtime, task};
use std::env;

mod request;
mod parser;
mod statistics;
mod webhook;
mod util;

#[tokio::main]
async fn main() {
    let threaded_rt = runtime::Runtime::new().unwrap();
    dotenv::dotenv().expect("Failed to load .env file");
    check_env();
    // Spawn worker for sending requests and shit
    let global_running = Arc::new(AtomicI32::new(0));
    let cloned_run = global_running.clone();

    let request_handle: task::JoinHandle<_> = threaded_rt.spawn(async move {
        loop {
            let html = request::send_request().await;
            if let Err(why) = html {
                println!("Error! {}", why);
                statistics::send_err_to_discord(String::from(why.to_string()).as_str()).await.unwrap();
                sleep(Duration::from_secs(30));
                continue;
            }

            let did_get = parser::parse_html(&html.unwrap());
            if did_get {
                util::send_success_message().await.unwrap();
            }
            println!("[SCRAPER {date}] [REQUEST] Count: {count} Did we get one? {didwegetone}",
                didwegetone = if did_get {"Yes"} else {"No"},
                date = Utc::now().with_timezone(&FixedOffset::east(25200)), // UTC+7:00 PST time i think
                count = global_running.fetch_add(1, Ordering::Relaxed)
            );
            sleep(Duration::from_secs(15));
        }
    });
    // Statistic Handler
    let stats_handler: task::JoinHandle<_> = threaded_rt.spawn(async move {
       loop {
           println!("[SCRAPER {}] [STATISTICS] Send statistics to Discord. Where we have {}.", Utc::now().with_timezone(&FixedOffset::east(25200)), cloned_run.load(Ordering::Relaxed)); // UTC+7:00 PST time i think);
           statistics::send_stats_to_bot(cloned_run.load(Ordering::Relaxed)).await;
           // reset statistics
           cloned_run.store(0, Ordering::Relaxed);
           sleep(Duration::from_secs(45));
       }
    });
    // await the handle so it doesn't end because retarded tokyo headass
    request_handle.await.expect("REQUEST HANDLE HAS ERRORED.");
    stats_handler.await.expect("STATISTICS HANDLE HAS ERRORED.");
}

fn check_env() {
    // Webhooks
    env::var("WEBHOOK_ID").expect("Expected a webhook id");
    env::var("WEBHOOK_TOKEN").expect("Expected a webhook token");

    // Bot
    env::var("BOT_TOKEN").expect("Expected a bot token");
    env::var("CHANNEL_ID").expect("Expected a channel id");
    env::var("MESSAGE_ID").expect("Expected a message id");
}