use reqwest::Error;
use reqwest::header;
use reqwest::Client;

use crate::suika::product::Response;
use crate::suika::product::ResponseCollection;

#[tokio::main]
pub async fn get() -> Result<ResponseCollection, Error> {
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
        .json::<Response>()
        .await?
        .result
        .total
        .to_string();


    let response: ResponseCollection = client
        .get("https://www.vinbudin.is/addons/origo/module/ajaxwebservices/search.asmx/DoSearch")
        .query(&[
            ("category", "beer"),
            ("count", &total),
            ("skip", "0")
        ])
        .send()
        .await?
        .json::<Response>()
        .await?
        .result;

    Ok(response)
}
