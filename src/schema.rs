table! {
    moves (id) {
        id -> Nullable<Integer>,
        san -> Text,
        from_id -> Integer,
        to_id -> Integer,
    }
}

table! {
    positions (id) {
        id -> Nullable<Integer>,
        fen -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    moves,
    positions,
);
