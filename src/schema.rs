// @generated automatically by Diesel CLI.

diesel::table! {
    clients (client_id) {
        client_id -> Nullable<Integer>,
        active -> Bool,
        username -> Text,
        pwd -> Text,
        birth_date -> Date,
    }
}
