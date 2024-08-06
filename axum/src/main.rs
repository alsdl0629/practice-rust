use axum::{routing::get, Router, Json, ServiceExt};
use axum::response::IntoResponse;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello_world));

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap()
}

pub async fn hello_world() -> impl IntoResponse {
    let response = serde_json::json!({
        "status": "OK",
        "message": "Hello World!"
    });

    return Json(response)
}
