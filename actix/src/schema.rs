table! {
    posts (id) {
        id -> Int4,
        title -> Text,
        content -> Text,
        published -> Bool,
        author_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Text,
        name -> Text,
    }
}

joinable!(posts -> users (author_id));

allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
