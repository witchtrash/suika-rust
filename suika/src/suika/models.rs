use diesel::{Queryable};

#[derive(Queryable)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub bottled_volume: f32,
    pub alcohol_by_volume: f32,
    pub price: f32,
    pub country_of_origin: String,
    pub is_available: bool,
    pub container_type: String,
    pub style: String,
    pub sub_style: String,
    pub producer: String,
    pub short_description: String,
    pub date_on_market: String,
    pub season: String,
}

#[derive(Queryable)]
pub struct ProductPrice {
    pub id: u32,
    pub products_id: u32,
    pub price: f32,
    pub timestamp: u64,
}
