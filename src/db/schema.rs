// @generated automatically by Diesel CLI.

diesel::table! {
    model (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    price (id) {
        id -> Int4,
        product_id -> Varchar,
        value -> Float8,
        date -> Timestamp,
    }
}

diesel::table! {
    product (id) {
        id -> Varchar,
        model_id -> Int4,
        product_name -> Varchar,
    }
}

diesel::joinable!(price -> product (product_id));
diesel::joinable!(product -> model (model_id));

diesel::allow_tables_to_appear_in_same_query!(
    model,
    price,
    product,
);
