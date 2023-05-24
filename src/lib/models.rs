use super::schema::{posts, posts_tags, tags};
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Identifiable, PartialEq, Debug)]
#[diesel(table_name = posts)]
pub struct Post {
    pub id: i32,
    pub img_path: Option<String>,
    pub title: Option<String>,
    pub source: Option<String>,
    pub posted_at: NaiveDateTime,
    pub score: i32,
}


#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost {
    pub title: Option<String>,
    pub source: Option<String>,
    pub posted_at: NaiveDateTime,
    pub score: i32,
}

#[derive(Queryable, Selectable, Identifiable, PartialEq, Debug)]
#[diesel(table_name = tags)]
pub struct Tag {
    pub id: i32,
    pub name: String,
    pub category: i32,
}

#[derive(Insertable)]
#[diesel(table_name = tags)]
pub struct NewTag {
    pub name: String,
    pub category: i32,
}

#[derive(Identifiable, Selectable, Queryable, Associations, Debug)]
#[diesel(belongs_to(Post))]
#[diesel(belongs_to(Tag))]
#[diesel(table_name = posts_tags)]
#[diesel(primary_key(post_id, tag_id))]
pub struct PostTag {
    pub post_id: i32,
    pub tag_id: i32,
}
