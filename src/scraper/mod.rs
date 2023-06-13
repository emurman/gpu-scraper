use diesel::{PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};

use crate::db::models::Model;

pub fn scrape(mut connection: PgConnection) -> () {
    use crate::db::schema::model::dsl::*;

    let result = model.select(Model::as_select()).load(&mut connection).expect("Failed to get models");

    for m in result {
        println!("{:?}", m);
    }
}

