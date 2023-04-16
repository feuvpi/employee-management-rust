table! {
    like (id) {
        if -> Uuid,
        created_at -> Timestamp,
        id_peep -> Uuid,
    }
}

table! {
    peeps (id) {
        id -> Uuid,
        created_at -> Timestamp,
        content -> Text,
    }
}

joinable!(likes -> peeps (id_peep));

allow_tables_to_appear_in_same_query!(likes, peeps);
