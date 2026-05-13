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
