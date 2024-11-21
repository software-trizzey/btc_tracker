mod database;

use error_chain::error_chain;

use reqwest::header::{HeaderMap, HeaderValue, InvalidHeaderValue};
use reqwest::Client;
use rusqlite::Connection;
use serde::Deserialize;
use tokio;
use dotenv::dotenv;
use std::env;


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
async fn main() -> Result<()> {
    dotenv().ok();

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
    println!("{:?}", data.bitcoin);

    let conn = Connection::open("btc_tracker.db")?;
    database::create_database(&conn)?;
    let currency_id = database::insert_currency(&conn, &data.bitcoin)?;
    println!("Inserted currency with ID: {}", currency_id);
    database::insert_quote(&conn, currency_id, &data.bitcoin.quote.usd)?;
    println!("Inserted quote for currency with ID: {}", currency_id);

    Ok(())
}
