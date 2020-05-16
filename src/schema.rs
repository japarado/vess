table! {
    comments (id) {
        id -> Int4,
        contents -> Text,
        user_id -> Int4,
        post_id -> Int4,
    }
}

table! {
    post_tags (post_id, tag_id) {
        post_id -> Int4,
        tag_id -> Int4,
    }
}

table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Nullable<Text>,
        user_id -> Int4,
    }
}

table! {
    tags (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        password -> Varchar,
    }
}

joinable!(comments -> posts (post_id));
joinable!(comments -> users (user_id));
joinable!(post_tags -> posts (post_id));
joinable!(post_tags -> tags (tag_id));
joinable!(posts -> users (user_id));

allow_tables_to_appear_in_same_query!(comments, post_tags, posts, tags, users,);
