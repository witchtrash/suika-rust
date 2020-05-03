extern crate reqwest;

use reqwest::Error;
use reqwest::header;
use serde_json::Value;


#[tokio::main]
pub async fn get() -> Result<Value, Error> {
    let client = reqwest::Client::new();

    let res: Value = client
        .get("https://httpbin.org/anything")
        .header(header::ACCEPT, "application/json")
        .header(header::CONTENT_TYPE, "application/json")
        .send()
        .await?
        .json()
        .await?;

    println!("{}", res);
    Ok(res)
}
