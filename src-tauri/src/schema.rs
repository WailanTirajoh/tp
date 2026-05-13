// @generated automatically by Diesel CLI.

diesel::table! {
    documents (key) {
        key -> Text,
        collection -> Nullable<Text>,
        value -> Text,
        version -> BigInt,
        created_at -> Text,
        updated_at -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        name -> Text,
        email -> Text,
        age -> Nullable<Integer>,
        created_at -> Text,
        updated_at -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(documents, users,);
