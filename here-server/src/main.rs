use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

// curl -X POST -H "Content-Type: application/json" -d '{"uuid":"xxxx", "data":"12312"}' 0.0.0.0:3000
// curl -X GET -H "Content-Type: application/json" -d '{"uuid":"xxxx", "data":"12312"}' 0.0.0.0:3000

#[derive(Deserialize)]
struct GetFile {
    uuid: String,
}

#[derive(Deserialize)]
struct PutFile {
    uuid: String,
    data: String,
}

#[derive(Serialize)]
struct File {
    data: String,
    name: String,
}

async fn get_file(Json(payload): Json<GetFile>) -> (StatusCode, Json<File>) {
    let GetFile { uuid } = payload;
    log::info!("uuid: {}", uuid);
    (
        StatusCode::OK,
        Json(File {
            data: "123".to_string(),
            name: "123".to_string(),
        }),
    )
}

async fn put_file(Json(payload): Json<PutFile>) -> StatusCode {
    let PutFile { uuid, data } = payload;
    log::info!("uuid: {}", uuid);
    log::info!("data: {}", data);
    StatusCode::OK
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let endpoint = std::env::var("HERE_ENDPOINT").unwrap_or("0.0.0.0:3000".into());

    let app = Router::new()
        .route("/", get(get_file))
        .route("/", post(put_file));

    axum::Server::bind(&endpoint.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
