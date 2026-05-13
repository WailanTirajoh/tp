// @generated automatically by Diesel CLI.

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
