use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Product {
    #[serde(rename = "ProductID")]
    pub id: u32,
    #[serde(rename = "ProductName")]
    pub name: String,
    #[serde(rename = "ProductBottledVolume")]
    pub bottled_volume: f32,
    // This typo is part of the API response :+1:
    #[serde(rename = "ProductAlchoholVolume")]
    pub alcohol_by_volume: f32,
    #[serde(rename = "ProductPrice")]
    pub price: f32,
    #[serde(rename = "ProductCountryOfOrigin")]
    pub country_of_origin: String,
    #[serde(rename = "ProductIsAvailableInStores")]
    pub is_available: bool,
    #[serde(rename = "ProductContainerType")]
    pub container_type: String,
    #[serde(rename = "ProductTasteGroup")]
    pub style: String,
    #[serde(rename = "ProductTasteGroup2")]
    pub sub_style: String,
    #[serde(rename = "ProductProducer")]
    pub producer: String,
    #[serde(rename = "ProductShortDescription")]
    pub short_description: String,
    #[serde(rename = "ProductDateOnMarket")]
    pub date_on_market: String,
    #[serde(rename = "ProductSeasonCode")]
    pub season: String,
}

#[derive(Deserialize, Debug)]
pub struct ResponseCollection {
    pub data: Vec<Product>,
    pub total: u32,
}

#[derive(Deserialize, Debug)]
pub struct Response {
    #[serde(rename = "d", with = "serde_with::json::nested")]
    pub result: ResponseCollection,
}
