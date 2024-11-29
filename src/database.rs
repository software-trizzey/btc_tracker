use rusqlite::{Connection, Result, params};
use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct Quote {
    pub price: f64,
    percent_change_24h: f64,
    last_updated: String,
}

#[derive(Deserialize, Debug)]
pub struct Quotes {
    #[serde(rename = "USD")]
    pub usd: Quote
}

#[derive(Deserialize, Debug)]
pub struct Cryptocurrency {
    id: u64,
    name: String,
    symbol: String,
    pub quote: Quotes,
}

pub fn create_database(connection: &Connection) -> Result<()> {
    connection.execute(
        "CREATE TABLE IF NOT EXISTS currency (
            id    INTEGER PRIMARY KEY,
            name  TEXT NOT NULL UNIQUE,
            symbol TEXT NOT NULL UNIQUE,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )",
        (),
    )?;

    connection.execute(
        "CREATE TABLE IF NOT EXISTS quote (
            id    INTEGER PRIMARY KEY,
            price  REAL NOT NULL,
            percent_change_24h REAL NOT NULL,
            currency_id INTEGER NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            last_updated TIMESTAMP NOT NULL
        )",
        (),
    )?;

    Ok(())
}

pub fn insert_currency(connection: &Connection, currency: &Cryptocurrency) -> Result<u64> {
    let mut stmt = connection.prepare("SELECT id FROM currency WHERE name = ?1 AND symbol = ?2")?;
    let mut rows = stmt.query(params![&currency.name, &currency.symbol])?;

    if let Some(row) = rows.next()? {
        // Currency already exists, return its ID
        let id: u64 = row.get(0)?;
        return Ok(id);
    }

    connection.execute(
        "INSERT INTO currency (id, name, symbol) VALUES (?1, ?2, ?3)",
        params![&currency.id, &currency.name, &currency.symbol],
    )?;

    let last_id = connection.last_insert_rowid() as u64;
    Ok(last_id)
}

pub fn insert_quote(connection: &Connection, currency_id: u64, quote: &Quote) -> Result<()> {
    connection.execute(
        "INSERT INTO quote (currency_id, last_updated, price, percent_change_24h) VALUES (?1, ?2, ?3, ?4)",
        params![currency_id, &quote.last_updated, &quote.price, &quote.percent_change_24h],
    )?;

    Ok(())
}