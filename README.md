# BTC Tracker

<img src="tracker_demo.png" alt="Program demo image" width="420px"/>


A Rust project to track Bitcoin prices and store them in an SQLite database.
Currency data is fetched from [CoinMarketCap](https://coinmarketcap.com/).


## Getting Started

### Prerequisites

- Rust and Cargo installed
- SQLite installed
- `dotenv` crate installed

### Installation

1. Clone the repository:

   ```sh
   git clone https://github.com/software-trizzey/btc_tracker.git
   cd btc_tracker
   ```
2. Create a `.env` file in the root of the project with the following content:
   **Note:** You'll need to sign up for a free account at [CoinMarketCap](https://coinmarketcap.com/) to get an API key.

   ```env
   API_KEY=your_api_key_here
   API_URL=https://pro-api.coinmarketcap.com/v2/cryptocurrency/quotes/latest
   ```
3. Build and run the project:

   ```sh
   cargo run
   ```
4. When ready, you can create a production version:

   ```sh
   cargo build --release
   ```
5. You can run the production version:
    
    ```sh
    ./target/debug/btc_tracker
    ```

    Note: once built you can move this binary where ever you want. Which means you can run it from a more convienient place.

### Inspecting the Database

Once the project is run, the SQLite database will be created and populated with data. The database file is named `btc_tracker.db`.

#### Using the SQLite Command-Line Interface

1. Open a terminal.
2. Run the SQLite command-line interface with the database file:

   ```sh
   sqlite3 btc_tracker.db
   ```
3. Once in the SQLite shell, you can run SQL commands to inspect the database. For example:

   ```sql
   -- Get the 25 most recent quotes
   select q.id as 'quote_id', q.price, q.percent_change_24h, q.created_at, c.name, c.symbol
   from quote as q
   inner join currency as c
   on q.currency_id = c.id
   order by
   date(q.created_at) DESC Limit 25;
   ```

#### Using a Graphical Tool

You can use graphical tools like DB Browser for SQLite to open and inspect the database file. These tools provide a user-friendly interface to view and manipulate the database contents.

## References

- [CoinMarketCap API docs](https://coinmarketcap.com/api/documentation/v1/#operation/getV2CryptocurrencyOhlcvLatest)