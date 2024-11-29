use error_chain::error_chain;
use reqwest::header::{HeaderMap, HeaderValue, InvalidHeaderValue};
use reqwest::Client;
use rusqlite::Connection;
use serde::Deserialize;
use serde_json::json;

use tokio::time::{interval, Duration};
use tokio::signal;
use tokio::sync::watch;
use std::sync::{
    Arc, 
    atomic::{AtomicBool, Ordering},
};
use std::env;
use colored::*;


mod database;


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


#[tokio::main]
pub async fn run(interval_duration: Duration) {
    let (tx, mut rx) = watch::channel(());
    let is_running = Arc::new(AtomicBool::new(true));
    let is_task_running = is_running.clone();

    let handle_task = tokio::spawn(async move {
        let mut interval = interval(interval_duration);

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
}


async fn fetch_and_store_latest_quote() -> Result<()> {
    println!("Fetching latest quote...");

    let data = get_bitcoin_quote().await?;

    let conn = Connection::open("btc_tracker.db")?;
    database::create_database(&conn)?;
    let currency_id = database::insert_currency(&conn, &data.bitcoin)?;
    database::insert_quote(&conn, currency_id, &data.bitcoin.quote.usd)?;

    println!("{}", "Success! A new BTC quote has been saved to the database");

    match env::var("MINIMUM_BUY_PRICE") {
        Ok(min_price) => {
            let parsed_minimum_price: f64 = match min_price.parse::<f64>() {
                Ok(parsed_price) => parsed_price,
                Err(_) => {
                    eprintln!("Error parsing MINIMUM_BUY_PRICE of \"{}\"! Is it a valid number?", min_price);
                    println!("Skipping notification.");
                    return Ok(());
                }
            };

            let price = data.bitcoin.quote.usd.price;

            if price <= parsed_minimum_price {
                println!("Heads up -- Quote ${} meets buy threshold ${}!", price, min_price);
                send_discord_message(&price, &min_price).await?;
            }
        },
        Err(_) => println!("No buy price set... Skipping notification."),
    };

    Ok(())
}

async fn get_bitcoin_quote() -> Result<Data> {
    let api_key = env::var("API_KEY")?;
    let api_url = env::var("API_URL")?;

    if api_key.is_empty() || api_url.is_empty() {
        panic!("API_KEY or API_URL is not set");
    }

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
    let mut data = api_response.data;
    println!("Found new BTC quote. Saving to database...");

    let formatted_price = (data.bitcoin.quote.usd.price * 100.0).round() / 100.0;
    data.bitcoin.quote.usd.price = formatted_price;

    Ok(data)
}

async fn send_discord_message(price: &f64, price_threshold: &str) -> Result<()> {
    let discord_url = env::var("DISCORD_URL")?;
    let discord_client = reqwest::Client::new();

    let payload = json!({
        "content": format!(
            "BTC quote **${}** meets your buy threshold of **${} USD**.\n_Pull the trigger!_",
            &price, &price_threshold
        )
    });

    let response = discord_client.post(&discord_url)
        .json(&payload)
        .send()
        .await;

    match response {
        Ok(_) => println!("Discord message sent!"),
        Err(err) => println!("Error: {}", err),
    }

    Ok(())
}