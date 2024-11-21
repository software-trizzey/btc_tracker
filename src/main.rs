mod database;

use error_chain::error_chain;

use reqwest::header::{HeaderMap, HeaderValue, InvalidHeaderValue};
use reqwest::Client;
use rusqlite::Connection;
use serde::Deserialize;

use tokio::time::{interval, Duration};
use tokio::signal;
use tokio::sync::watch;
use dotenv::dotenv;
use std::sync::{
    Arc, 
    atomic::{AtomicBool, Ordering},
};
use std::env;
use colored::*;


error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
        InvalidHeaderValue(InvalidHeaderValue);
        JsonError(serde_json::Error);
        VarError(env::VarError);
        DatabaseError(rusqlite::Error);
    }
}

#[derive(Deserialize, Debug)]
struct Data {
    #[serde(rename = "1")]
    bitcoin: database::Cryptocurrency,
}

#[derive(Deserialize, Debug)]
struct ApiResponse {
    data: Data,
}

const THIRTY_MINUTES: Duration = Duration::from_secs(30 * 60);

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    println!("\n{}", "Starting BTC tracker (v1.0.0)...".bold());

    let (tx, mut rx) = watch::channel(());
    let is_running = Arc::new(AtomicBool::new(true));
    let is_task_running = is_running.clone();

    let handle_task = tokio::spawn(async move {
        let mut interval = interval(THIRTY_MINUTES);

        let mut task_count = 1;
        loop {
            tokio::select! {
                _ = interval.tick() => {
                    if !is_task_running.load(Ordering::Relaxed) {
                        break;
                    }
                    println!("\n{}", format!("Task #{} is running...", task_count).bold());
                    if let Err(e) = fetch_and_store_latest_quote().await {
                        eprintln!("Error fetching and storing quote: {}", e);
                        break;
                    }
                    println!("\n{}", format!("Task #{} completed successfully.", task_count).green().bold());
                    task_count += 1;

                    println!("\nWaiting for next task...");
                }
                _ = rx.changed() => {
                    break;
                }
            }
        }
    });

    println!("{}", "Stop program using: Ctrl+C".yellow());
    signal::ctrl_c().await.unwrap();
    println!("\n{}", "Ctrl+C pressed, stopping task immediately...".yellow());

    // Set the flag to stop the task
    is_running.store(false, Ordering::Relaxed);
    let _ = tx.send(());

    handle_task.await.unwrap();
    println!("\n{}", "Task fully stopped... Exiting program... Bye! 👋".green().bold());

    Ok(())
}


async fn fetch_and_store_latest_quote() -> Result<()> {
    println!("Fetching latest quote...");

    let api_key = env::var("API_KEY")?;
    let api_url = env::var("API_URL")?;

    let client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert("X-CMC_PRO_API_KEY", HeaderValue::from_str(&api_key)?);

    let bitcoin_url = format!("{}?slug=bitcoin", api_url);
    let res = client
        .get(&bitcoin_url)
        .headers(headers)
        .send()
        .await?;

    let body = res.text().await?;
    
    let api_response: ApiResponse = serde_json::from_str(&body)?;
    let data = api_response.data;
    println!("Found new BTC quote. Saving to database...");

    let conn = Connection::open("btc_tracker.db")?;
    database::create_database(&conn)?;
    let currency_id = database::insert_currency(&conn, &data.bitcoin)?;
    database::insert_quote(&conn, currency_id, &data.bitcoin.quote.usd)?;

    println!("{}", "Success! A new BTC quote has been saved to the database");

    Ok(())
}
