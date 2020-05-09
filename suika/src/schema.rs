table! {
    product_prices (id) {
        id -> Int4,
        products_id -> Int4,
        price -> Float4,
        timestamp -> Timestamp,
    }
}

table! {
    products (id) {
        id -> Int4,
        product_id -> Varchar,
        name -> Varchar,
        bottled_volume -> Float4,
        alcohol_by_volume -> Float4,
        country_of_origin -> Varchar,
        is_available -> Bool,
        container_type -> Varchar,
        style -> Varchar,
        sub_style -> Varchar,
        producer -> Varchar,
        short_description -> Varchar,
        date_on_market -> Date,
        season -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    product_prices,
    products,
);
