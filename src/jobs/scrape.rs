use reqwest::Error;
use reqwest::header;
use reqwest::Client;
use serde::Deserialize;
use serde_json::Value;
use serde_with;

#[derive(Deserialize, Debug)]
pub struct BeerResponse {
    #[serde(rename = "d", with = "serde_with::json::nested")]
    result: BeerData
}

#[derive(Deserialize, Debug)]
pub struct BeerData {
    data: Vec<Value>,
    total: u32
}


#[tokio::main]
pub async fn get() -> Result<BeerResponse, Error> {
    let mut headers = header::HeaderMap::new();
    headers.insert(header::ACCEPT, header::HeaderValue::from_static("application/json"));
    headers.insert(header::CONTENT_TYPE, header::HeaderValue::from_static("application/json"));

    let client = Client::builder()
        .default_headers(headers)
        .build()?;


    let response: BeerResponse = client
        .get("https://www.vinbudin.is/addons/origo/module/ajaxwebservices/search.asmx/DoSearch")
        .query(&[
            ("category", "beer"),
            ("count", "0"),
            ("skip", "0")
        ])
        .send()
        .await?
        .json()
        .await?;

    let total = response.result.total.to_string();

    let response: BeerResponse = client
        .get("https://www.vinbudin.is/addons/origo/module/ajaxwebservices/search.asmx/DoSearch")
        .query(&[
            ("category", "beer"),
            ("count", &format!("{}", &total)),
            ("skip", "0")
        ])
        .send()
        .await?
        .json()
        .await?;

    Ok(response)
}
