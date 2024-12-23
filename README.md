# BTC Tracker

<img src="tracker_demo.png" alt="Program demo image" width="620px"/>

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
   ./target/release/btc_tracker
   ```
6. Once you've built the binary, you can run move it to a more convenient place and run from there.

   Example: Running the program from the desktop

   ```sh
   # starts from the btc_tracker root
   cd target/release
   cp btc_tracker ~/Desktop
   chmod +x ~/Desktop/btc_tracker
   cd ~/Desktop
   ./btc_tracker
   ```

   Note: once you move the binary, you'll need a solution for setting the .env variables.

   Example (not recommended for prod/serious apps):

   ```sh
   export API_URL="http://example.com"
   export API_KEY="wow-such-secret-key-1213!"
   ./btc_tracker
   ```

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

### Compile the project for a Raspberry Pi

As a fun challenge, I wanted to run this project on an old Raspberry Pi 4 I had lying around.

Here are the steps I used

1. Install the Cross-Compilation Toolchain You'll need a cross-compilation toolchain to build for the Raspberry Pi:

```bash
rustup target add armv7-unknown-linux-gnueabihf
```

2. Compile Program for the target arch:

```bash
cross build --release --target armv7-unknown-linux-gnueabihf --package btc_tracker
```

3. Transfer to the Raspberry Pi

```bash
scp target/armv7-unknown-linux-gnueabihf/release/btc_tracker pi@<raspberry-pi-ip>:/home/pi/
```

4. Connect to Raspberry Pi via SSH and run program

```bash
ssh pi@<raspberry-pi-ip>
API_KEY=<add_your_key> API_URL=<add_quote_url> DISCORD_URL=<add_discord_webhook_url> MINIMUM_BUY_PRICE=<set_notify_price> ./btc_tracker
```


## References

- [CoinMarketCap API docs](https://coinmarketcap.com/api/documentation/v1/#operation/getV2CryptocurrencyOhlcvLatest)
