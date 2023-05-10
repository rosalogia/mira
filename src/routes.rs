use crate::lib::{establish_connection, models::*, schema};
use axum::extract::{Json, Multipart, Path, Query};
use axum_macros::debug_handler;
use chrono::{prelude::*, NaiveDateTime};
use diesel::dsl::count_distinct;
use diesel::prelude::*;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use serde::Deserialize;
use std::{fs::File, io::prelude::*};

#[derive(Deserialize)]
pub struct CreatePost {
    pub title: Option<String>,
    pub source: Option<String>,
    pub tags: Vec<String>,
}

pub struct ViewPost {
    pub id: i32,
    pub img_path: Option<String>,
    pub title: Option<String>,
    pub source: Option<String>,
    pub posted_at: NaiveDateTime,
    pub score: i32,
    pub tags: Vec<String>,
}

impl Serialize for ViewPost {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Post", 7)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("img_path", &self.img_path)?;
        state.serialize_field("title", &self.title)?;
        state.serialize_field("source", &self.source)?;
        state.serialize_field("posted_at", &self.posted_at.to_string())?;
        state.serialize_field("score", &self.score)?;
        state.serialize_field("tags", &self.tags)?;
        state.end()
    }
}

fn associate_post_tag(post_id: i32, tag: String) {
    use schema::posts::dsl::*;
    use schema::tags::dsl::*;

    let db_conn = &mut establish_connection();
    let post: Post = posts.find(post_id).first(db_conn).unwrap();
    let tag: Tag = match tags.filter(name.eq(&tag)).first(db_conn) {
        Ok(t) => t,
        Err(e) => {
            println!("Tag does not exist, creating new tag");
            let new_tag = NewTag {
                name: tag,
                category: -1,
            };

            diesel::insert_into(schema::tags::table)
                .values(new_tag)
                .returning(Tag::as_returning())
                .get_result(db_conn)
                .unwrap()
        }
    };

    {
        use schema::posts_tags::dsl::*;
        diesel::insert_into(schema::posts_tags::table)
            .values((post_id.eq(post.id), tag_id.eq(tag.id)))
            .returning(PostTag::as_returning())
            .get_result(db_conn)
            .unwrap();
    }
}

#[debug_handler]
pub async fn post_metadata_handler(
    Json(payload): Json<CreatePost>,
) -> Result<String, axum::http::StatusCode> {
    let utcn = Utc::now();
    let db_conn = &mut establish_connection();
    let CreatePost {
        title,
        source,
        tags,
    } = payload;

    let new_post = NewPost {
        title,
        source,
        posted_at: NaiveDateTime::new(utcn.date_naive(), utcn.time()),
        score: 0,
    };

    let post: Post = diesel::insert_into(schema::posts::table)
        .values(&new_post)
        .get_result(db_conn)
        .expect("Error saving new post");

    for tag in tags.into_iter() {
        associate_post_tag(post.id, tag)
    }

    Result::Ok(post.id.to_string())
}

#[debug_handler]
pub async fn post_img_handler(
    Path(post_id): Path<i32>,
    mut multipart: Multipart,
) -> Result<String, axum::http::StatusCode> {
    use crate::lib::schema::posts::dsl::*;

    let db_conn = &mut establish_connection();
    let post: Post = match posts.find(post_id).first(db_conn) {
        Ok(post) => post,
        Err(e) => {
            println!("{}", e);
            return Result::Err(axum::http::StatusCode::BAD_REQUEST);
        }
    };

    let mut new_file = File::create(post.id.to_string()).expect("Failed to create new image");
    if let Some(field) = multipart.next_field().await.unwrap() {
        new_file
            .write_all(&field.bytes().await.unwrap())
            .expect("Failed to write new file");
    }

    diesel::update(&post)
        .set(img_path.eq(Some(post.id.to_string())))
        .get_result::<Post>(db_conn)
        .unwrap();

    Result::Ok(String::from("Okay"))
}

#[debug_handler]
pub async fn view_post_handler(
    Path(post_id): Path<i32>,
) -> Result<Json<ViewPost>, axum::http::StatusCode> {
    use schema::posts::dsl::*;
    let db_conn = &mut establish_connection();

    let post: Post = match posts.find(post_id).first(db_conn) {
        Ok(post) => post,
        Err(e) => {
            println!("{}", e);
            return Result::Err(axum::http::StatusCode::BAD_REQUEST);
        }
    };

    let tags: Vec<Tag> = PostTag::belonging_to(&post)
        .inner_join(schema::tags::table)
        .select(Tag::as_select())
        .load(db_conn)
        .unwrap();

    let view_post = ViewPost {
        id: post.id,
        img_path: post.img_path,
        title: post.title,
        source: post.source,
        posted_at: post.posted_at,
        score: post.score,
        tags: tags.iter().map(|tag| tag.name.clone()).collect(),
    };

    Ok(Json(view_post))
}

#[derive(Deserialize)]
pub struct Search {
    tags: String,
}

#[debug_handler]
pub async fn search_posts_handler(
    query: Query<Search>,
) -> Result<Json<Vec<i32>>, axum::http::StatusCode> {
    use schema::posts::dsl::*;
    use schema::posts_tags::dsl::*;
    use schema::tags::dsl::*;

    let db_conn = &mut establish_connection();
    let tag_list: Vec<&str> = query.0.tags.split(',').collect();
    println!("tags: {:?}", tag_list);

    let post_list = posts
        .inner_join(posts_tags.on(post_id.eq(schema::posts::id)))
        .inner_join(tags.on(schema::tags::id.eq(tag_id)))
        .filter(name.eq_any(&tag_list))
        .group_by(schema::posts::id)
        .having(count_distinct(schema::tags::name).eq(tag_list.len() as i64))
        .select(schema::posts::id)
        .load::<i32>(db_conn)
        .unwrap();

    println!("Found posts: {:?}", post_list);

    Ok(Json(post_list))
}
