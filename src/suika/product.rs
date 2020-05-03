use serde_json::Value;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Product {
    pub placeholder: String
}

#[derive(Deserialize, Debug)]
pub struct ProductCollection {
    pub data: Vec<Value>,
    pub total: u32,
}

#[derive(Deserialize, Debug)]
pub struct ProductResponse {
    #[serde(rename = "d", with = "serde_with::json::nested")]
    pub result: ProductCollection,
}
