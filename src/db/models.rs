use diesel::{prelude::*};
use chrono::NaiveDateTime;

#[derive(Insertable, Debug, Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::model)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Model {
    pub name: String,
}

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::price)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Price {
    pub id: i32,
    pub value: f64,
    pub model_name: String,
    pub product_name: String,
    pub date: NaiveDateTime
}

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::db::schema::price)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewPrice {
    pub value: f64,
    pub model_name: String,
    pub product_name: String
}