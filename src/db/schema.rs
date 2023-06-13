// @generated automatically by Diesel CLI.

diesel::table! {
    model (name) {
        name -> Varchar,
    }
}

diesel::table! {
    price (id) {
        id -> Int4,
        model_name -> Varchar,
        product_name -> Varchar,
        value -> Float8,
        date -> Timestamp,
    }
}

diesel::joinable!(price -> model (model_name));

diesel::allow_tables_to_appear_in_same_query!(
    model,
    price,
);
