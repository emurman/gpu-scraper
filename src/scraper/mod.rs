use std::collections::HashMap;

use diesel::{PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};
use serde::Deserialize;

use crate::db::models::{Model, NewPrice};

const FIND_PRODUCT_IDS_URL: &str = "https://www.inet.se/api/filter/v2?wh=00&includeHiddenFilters=false&companyMode=false&sortColumn=search&sortDirection=desc";
const SCRAPE_PRICES_URL: &str = "https://www.inet.se/api/products";

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
struct ProductIdsScrapeResponse {
    productIds: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct ProductPricesScrapeResponse {
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

    for m in result {
        scrape_model(m, &mut connection)
    }
}

fn scrape_model(m: Model, conn: &mut PgConnection) {
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

    for product_price in prices_response.into_values() {
        let new_price = NewPrice {
            model_name: m.name.clone(),
            product_name: product_price.name,
            value: product_price.price.price,
        };
        let result = diesel::insert_into(price::table)
            .values(&new_price)
            .returning(crate::db::models::Price::as_returning())
            .get_result(conn)
            .expect("Failed to push price");

        // todo: log how many updates were performed etc
    }
}

fn create_payload(model_name: &String) -> String {
    format!(
        r#"{{"userSelectedFilters":{{"propertyFilters":{{"text":{{"29":["{}"]}}}},"manufacturerIds":[]}},"hiddenFilters":{{"templateIds":[17],"categoryIds":[167],"includeManufacturerFacets":true,"isActive":true,"isHidden":false}}}}"#,
        model_name
    )
}
