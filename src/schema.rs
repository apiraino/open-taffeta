table! {
    doors (id) {
        id -> Integer,
        name -> Text,
        address -> Text,
        ring -> Bool,
        ring_ts -> Nullable<Integer>,
    }
}

table! {
    users (id) {
        id -> Integer,
        password -> Text,
        email -> Text,
        active -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    doors,
    users,
);
