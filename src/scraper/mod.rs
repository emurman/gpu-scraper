use std::collections::HashMap;

use diesel::{PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};
use serde::Deserialize;

use crate::db::{
    models::{Model, NewPrice, Product},
    schema::product,
};

const FIND_PRODUCT_IDS_URL: &str = "https://www.inet.se/api/filter/v2?wh=00&includeHiddenFilters=false&companyMode=false&sortColumn=search&sortDirection=desc";
const SCRAPE_PRICES_URL: &str = "https://www.inet.se/api/products";

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
struct ProductIdsScrapeResponse {
    productIds: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct ProductPricesScrapeResponse {
    id: String,
    name: String,
    price: Price,
}

#[derive(Debug, Deserialize)]
struct Price {
    price: f64,
}

pub fn scrape(mut connection: PgConnection) -> () {
    use crate::db::schema::model::dsl::*;

    let result = model
        .select(Model::as_select())
        .load(&mut connection)
        .expect("Failed to get models");

    let mut count = 0;
    for m in result {
        count += scrape_model(m, &mut connection)
    }

    println!("Successfully fetched prices for {} products", count);
}

fn scrape_model(m: Model, conn: &mut PgConnection) -> u32 {
    use crate::db::schema::price;

    let client = reqwest::blocking::Client::new();
    let product_ids = client
        .post(FIND_PRODUCT_IDS_URL)
        .body(create_payload(&m.name))
        .header("Content-Type", "application/json")
        .send()
        .expect(format!("Failed to fetch data for {}", m.name).as_str())
        .json::<ProductIdsScrapeResponse>()
        .expect(format!("Failed to parse response for {}", m.name).as_str())
        .productIds;

    let prices_response = client
        .post(SCRAPE_PRICES_URL)
        .body(format!("{:?}", product_ids))
        .header("Content-Type", "application/json")
        .send()
        .expect(format!("Failed to fetch data for {}", m.name).as_str())
        .json::<HashMap<String, ProductPricesScrapeResponse>>()
        .expect(format!("Failed to parse response for {}", m.name).as_str());

    let result = prices_response.len() as u32;
    for product_price in prices_response.into_values() {
        diesel::insert_into(product::table)
            .values(Product {
                id: product_price.id.clone(),
                model_id: m.id,
                product_name: product_price.name,
            })
            .on_conflict_do_nothing()
            .execute(conn).expect("Failed to upsert product");

        let new_price = NewPrice {
            product_id: product_price.id,
            value: product_price.price.price,
        };
        let _ = diesel::insert_into(price::table)
            .values(&new_price)
            .returning(crate::db::models::Price::as_returning())
            .get_result(conn)
            .expect("Failed to push price");
    }

    result
}

fn create_payload(model_name: &String) -> String {
    format!(
        r#"{{"userSelectedFilters":{{"propertyFilters":{{"text":{{"29":["{}"]}}}},"manufacturerIds":[]}},"hiddenFilters":{{"templateIds":[17],"categoryIds":[167],"includeManufacturerFacets":true,"isActive":true,"isHidden":false}}}}"#,
        model_name
    )
}
