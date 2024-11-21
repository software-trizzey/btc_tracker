use error_chain::error_chain;

use reqwest::header::{HeaderMap, HeaderValue, InvalidHeaderValue};
use reqwest::Client;
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
    }
}

#[derive(Deserialize, Debug)]
struct Quote {
    price: f64,
}

#[derive(Deserialize, Debug)]
struct Quotes {
    #[serde(rename = "USD")]
    usd: Quote
}

#[derive(Deserialize, Debug)]
struct Cryptocurrency {
    id: u64,
    name: String,
    symbol: String,
    quote: Quotes,
}

#[derive(Deserialize, Debug)]
struct Data {
    #[serde(rename = "1")]
    bitcoin: Cryptocurrency,
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
    // TODO: parse the response and store the data in a struct
    println!("{:?}", data);

    Ok(())
}
