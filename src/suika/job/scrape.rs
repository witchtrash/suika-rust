use reqwest::Error;
use reqwest::header;
use reqwest::Client;

use crate::suika::product::ProductResponse;

#[tokio::main]
pub async fn get() -> Result<ProductResponse, Error> {
    let mut headers = header::HeaderMap::new();
    headers.insert(header::ACCEPT, header::HeaderValue::from_static("application/json"));
    headers.insert(header::CONTENT_TYPE, header::HeaderValue::from_static("application/json"));

    let client = Client::builder()
        .default_headers(headers)
        .build()?;


    let total: String = client
        .get("https://www.vinbudin.is/addons/origo/module/ajaxwebservices/search.asmx/DoSearch")
        .query(&[
            ("category", "beer"),
            ("count", "0"),
            ("skip", "0")
        ])
        .send()
        .await?
        .json::<ProductResponse>()
        .await?
        .result
        .total
        .to_string();


    let response: ProductResponse = client
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
