use dotenv::dotenv;
use tokio::time::Duration;

use colored::*;


const THIRTY_MINUTES: Duration = Duration::from_secs(30 * 60);

fn main() {
    dotenv().ok();

    println!("\n{}", "Welcome to BTC tracker (v1.0.0)!".bold());

    let formatted_time = format!("{} minutes", THIRTY_MINUTES.as_secs() / 60).italic();
    println!(
        "\nThis program will fetch the latest BTC quote every {} and store it in an SQLite database.",
        formatted_time
    );
    println!("\nPlease review the README.md to learn how to access the database");

    // TODO: add command line option to control notification logic (email instead of discord etc.)

    btc_tracker::run(THIRTY_MINUTES);
}

