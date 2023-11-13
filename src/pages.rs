use axum::{
    body::{boxed, Body, BoxBody},
    http::{Request, StatusCode, Uri},
    response::{Html, Response},
    routing::get,
    Router,
};
use tower::ServiceExt;
use tower_http::services::ServeDir;

pub fn router() -> Router {
    Router::new()
        .route("/", get(root_get))
        .nest_service("/static", get(static_path_get))
}

// #[axum::debug_handler]
// async fn root_get() -> Html<&'static str> {
//     Html(include_str!("index.html"))
// }

#[axum::debug_handler]
async fn root_get() -> Html<String> {
    let markup = tokio::fs::read_to_string("src/index.html").await.unwrap();
    Html(markup)
}

async fn static_path_get(
    uri: Uri,
) -> Result<Response<BoxBody>, (StatusCode, String)> {
    let req = Request::builder().uri(uri).body(Body::empty()).unwrap();

    match ServeDir::new("./static").oneshot(req).await {
        Ok(res) => Ok(res.map(boxed)),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", err),
        )),
    }
}
