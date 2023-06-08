use std::cell::RefCell;

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
    name: String,
    data: String,
}

#[derive(Serialize, Clone)]
struct File {
    data: String,
    name: String,
}

thread_local! {
    static CURRENT_FILE: RefCell<Option<File>> = RefCell::new(None);
}

async fn get_file(Json(payload): Json<GetFile>) -> (StatusCode, Json<File>) {
    let GetFile { uuid } = payload;
    log::info!("uuid: {}", uuid);
    CURRENT_FILE.with(|f| {
        if let Some(file) = &*f.borrow() {
            if file.name == uuid {
                return (StatusCode::OK, Json(file.clone()));
            }
        }
        (
            StatusCode::NOT_FOUND,
            Json(File {
                data: "".to_string(),
                name: "".to_string(),
            }),
        )
    })
}

async fn put_file(Json(payload): Json<PutFile>) -> StatusCode {
    let PutFile { uuid, name, data } = payload;
    log::info!("uuid: {uuid}");
    log::info!("name: {name}");
    log::info!("data: {data}");
    CURRENT_FILE.with(|f| *f.borrow_mut() = Some(File { data, name }));
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
