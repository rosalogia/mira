use crate::lib::parser;
use crate::lib::{establish_connection, models::*, schema};
use axum::extract::{Json, Path, Query};
use axum_macros::debug_handler;
use chrono::NaiveDateTime;
use diesel::sql_types::Text;
use diesel::{prelude::*, sql_query};
use serde::ser::{Serialize, SerializeStruct, Serializer};
use serde::Deserialize;

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

#[derive(serde::Serialize, Debug)]
pub struct View {
    pub id: i32,
    pub img_path: String,
}

#[debug_handler]
pub async fn view() -> Result<Json<Vec<View>>, axum::http::StatusCode> {
    use schema::posts::dsl::*;
    let db_conn = &mut establish_connection();
    println!("Viewing");
    let images: Vec<View> = posts
        .filter(img_path.is_not_null())
        .select((id, img_path))
        .order(posted_at.desc())
        .load::<(i32, Option<String>)>(db_conn)
        .unwrap()
        .into_iter()
        .map(|(i, path)| View {
            id: i,
            img_path: path.unwrap(),
        })
        .collect();

    Ok(Json(images))
}

#[debug_handler]
pub async fn view_id(Path(post_id): Path<i32>) -> Result<Json<ViewPost>, axum::http::StatusCode> {
    use schema::posts::dsl::*;
    let db_conn = &mut establish_connection();

    println!("Looking for post: {}", post_id);
    let post: Post = match posts.find(post_id).first(db_conn) {
        Ok(post) => post,
        Err(e) => {
            println!("{}", e);
            return Result::Err(axum::http::StatusCode::BAD_REQUEST);
        }
    };

    let tags: Vec<String> = PostTag::belonging_to(&post)
        .inner_join(schema::tags::table)
        .select(schema::tags::name)
        .order(schema::tags::name.asc())
        .load(db_conn)
        .unwrap();

    let view_post = ViewPost {
        id: post.id,
        img_path: post.img_path,
        title: post.title,
        source: post.source,
        posted_at: post.posted_at,
        score: post.score,
        tags: tags.iter().map(|tag| tag.into()).collect(),
    };

    Ok(Json(view_post))
}

#[derive(Deserialize)]
pub struct Search {
    tags: String,
}

#[debug_handler]
pub async fn search(query: Query<Search>) -> Result<Json<Vec<View>>, axum::http::StatusCode> {
    let db_conn = &mut establish_connection();
    println!("{}", &query.0.tags);
    let search_query = match parser::parse_boolean_expression(&query.0.tags) {
        Ok(query) => {
            println!("{:?}", query);
            query
        }
        Err(e) => {
            println!("{:?}", e);
            return Err(axum::http::StatusCode::BAD_REQUEST);
        }
    };

    let (queries, values) = search_query.to_sql();
    println!("{}", queries);

    let query = format!(
        "SELECT p.id, p.img_path, p.title, p.source, p.posted_at, p.score FROM posts p \
        INNER JOIN posts_tags pt ON pt.post_id = p.id \
        INNER JOIN tags t ON t.id = pt.tag_id \
        GROUP BY p.id HAVING {}",
        queries
    );

    println!("{}", query);

    let query = values
        .into_iter()
        .fold(sql_query(query).into_boxed(), |q, v| q.bind::<Text, _>(v));

    let post_list: Vec<View> = match query.get_results::<Post>(db_conn) {
        Ok(pl) => pl
            .into_iter()
            .map(|p| View {
                id: p.id,
                img_path: p.img_path.unwrap(),
            })
            .collect(),
        Err(e) => {
            println!("SQL query failure: {:?}", e);
            return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    Ok(Json(post_list))
}
