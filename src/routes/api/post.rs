use crate::lib::{establish_connection, models::*, schema};
use axum::{
    body::Bytes,
    extract::{Json, Multipart, Path},
};
use axum_macros::debug_handler;
use chrono::{prelude::*, NaiveDateTime};
use diesel::prelude::*;
use serde::Deserialize;
use std::{fs::File, io::{prelude::*, ErrorKind}};

#[derive(Deserialize)]
pub struct CreatePost {
    pub title: Option<String>,
    pub source: Option<String>,
    pub score: Option<i32>,
    pub tags: Vec<String>,
}

async fn create_image(file_name: String, file_contents: &Bytes) -> std::io::Result<()> {
    let acceptable_extensions = vec![".png", ".jpg", ".jpeg", ".gif"];
    if acceptable_extensions
        .iter()
        .any(|ending| file_name.ends_with(ending))
    {
        File::create(format!("./images/{}", file_name))?
            .write_all(file_contents)
    } else {
        Err(std::io::Error::new(ErrorKind::InvalidInput, "Filename must end in .png, .jpg, .jpeg, or .gif"))
    }
}

fn associate_post_tag(post_id: i32, tag: String) {
    use schema::posts::dsl::*;
    use schema::tags::dsl::*;

    let db_conn = &mut establish_connection();
    let post: Post = posts.find(post_id).first(db_conn).unwrap();
    let tag: Tag = match tags.filter(name.eq(&tag)).first(db_conn) {
        Ok(t) => t,
        Err(_) => {
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
pub async fn metadata(Json(payload): Json<CreatePost>) -> Result<String, axum::http::StatusCode> {
    let utcn = Utc::now();
    let db_conn = &mut establish_connection();
    let CreatePost {
        title,
        source,
        score,
        tags,
    } = payload;

    let new_post = NewPost {
        title,
        source,
        posted_at: NaiveDateTime::new(utcn.date_naive(), utcn.time()),
        score: score.unwrap(),
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
pub async fn image(
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

    if let Some(field) = multipart.next_field().await.unwrap() {
        let file_name = field.file_name().unwrap().to_string();
        let file_contents = field.bytes().await.unwrap();
        match create_image(
            file_name.clone(),
            &file_contents
        ).await {
            Ok(_) => {
                diesel::update(&post)
                    .set(img_path.eq(Some(format!("/images/{}", file_name))))
                    .get_result::<Post>(db_conn)
                    .unwrap();

                Result::Ok(String::from("Okay"))
            },
            Err(_) => Result::Err(axum::http::StatusCode::BAD_REQUEST)
        }
    } else {
        Err(axum::http::StatusCode::BAD_REQUEST)
    }

}
