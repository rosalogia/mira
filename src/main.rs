pub mod lib;
mod routes;
use axum::{
    routing::{get, post},
    Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello world" }))
        .route("/post/meta", post(routes::post_metadata_handler))
        .route("/post/image/:post_id", post(routes::post_img_handler))
        .route("/view/:post_id", get(routes::view_post_handler))
        .route("/search", get(routes::search_posts_handler));


    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
