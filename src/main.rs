mod models;

use dotenv::dotenv;
use models::*;
use reqwest::header::USER_AGENT;
use tokio::main;

#[main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let url = "https://discord.store/products.json?limit=250";

    let client = reqwest::Client::new();

    let body = client
        .get(url)
        .header(USER_AGENT, "https://github.com/Datacord-GH/store-tracker")
        .send()
        .await
        .unwrap();

    let products = body.json::<ProductRespose>().await.unwrap();

    println!("{:#?}", products.products.first());

    Ok(())
}
