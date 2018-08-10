table! {
    doors (id) {
        id -> Integer,
        name -> Text,
        rung -> Bool,
    }
}

table! {
    users (id) {
        id -> Integer,
        username -> Text,
        password -> Text,
        email -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    doors,
    users,
);
