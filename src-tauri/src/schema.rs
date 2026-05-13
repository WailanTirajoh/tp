// @generated automatically by Diesel CLI.

diesel::table! {
    audit_logs (id) {
        id -> Integer,
        table_name -> Text,
        record_id -> Integer,
        action -> Text,
        old_values -> Nullable<Text>,
        new_values -> Nullable<Text>,
        user_id -> Nullable<Integer>,
        created_at -> Text,
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

diesel::allow_tables_to_appear_in_same_query!(
    audit_logs,
    users,
);
