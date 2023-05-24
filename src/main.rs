pub mod lib;
pub mod routes;
use axum::{
    routing::{get, post},
    Router, extract::DefaultBodyLimit, http::header::CONTENT_TYPE,
};

#[tokio::main]
async fn main() {
    let browse_routes = Router::new()
        .route("/view", get(routes::api::browse::view))
        .route("/view/:post_id", get(routes::api::browse::view_id))
        .route("/search", get(routes::api::browse::search));

    let post_routes = Router::new()
        .route("/meta", post(routes::api::post::metadata))
        .route("/image/:post_id", post(routes::api::post::image))
        .layer(DefaultBodyLimit::disable());

    let api_routes =  Router::new()
        .nest("/browse", browse_routes)
        .nest("/post", post_routes);

    let app = Router::new()
        .nest("/api", api_routes)
        .nest("/static", axum_static::static_router("static"))
        .layer(
            tower_http::cors::CorsLayer::new()
                .allow_origin("http://localhost:4000".parse::<axum::http::HeaderValue>().unwrap())
                .allow_headers([CONTENT_TYPE])
                .allow_methods([axum::http::Method::GET, axum::http::Method::POST])
        );


    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
