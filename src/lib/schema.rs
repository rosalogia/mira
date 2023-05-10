// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Int4,
        img_path -> Nullable<Varchar>,
        title -> Nullable<Varchar>,
        source -> Nullable<Varchar>,
        posted_at -> Timestamp,
        score -> Int4,
    }
}

diesel::table! {
    posts_tags (post_id, tag_id) {
        post_id -> Int4,
        tag_id -> Int4,
    }
}

diesel::table! {
    tags (id) {
        id -> Int4,
        name -> Varchar,
        category -> Int4,
    }
}

diesel::joinable!(posts_tags -> posts (post_id));
diesel::joinable!(posts_tags -> tags (tag_id));

diesel::allow_tables_to_appear_in_same_query!(
    posts,
    posts_tags,
    tags,
);
