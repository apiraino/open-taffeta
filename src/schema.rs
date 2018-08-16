// TODO: investigate why it specifies again `id` as non_standard_primary_key, also two (id) conflicts
// https://docs.diesel.rs/diesel/macro.table.html

table! {
    doors (id) {
        id -> Integer,
        name -> Text,
        ring -> Bool,
        ring_ts -> Nullable<Integer>,
    }
}

table! {
    users (id) {
        id -> Integer,
        username -> Text,
        password -> Text,
        email -> Text,
        active -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(doors, users,);
