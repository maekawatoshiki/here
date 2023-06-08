use std::{cell::RefCell, sync::Mutex};

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
    uuid: String,
    data: String,
    name: String,
}

static CURRENT_FILE: Mutex<RefCell<Option<File>>> = Mutex::new(RefCell::new(None));

async fn get_file(Json(payload): Json<GetFile>) -> (StatusCode, Json<File>) {
    let GetFile { uuid } = payload;
    log::info!("uuid: {}", uuid);
    CURRENT_FILE.lock().unwrap().borrow().as_ref().map_or_else(
        || {
            (
                StatusCode::NOT_FOUND,
                Json(File {
                    uuid: "".to_string(),
                    data: "".to_string(),
                    name: "".to_string(),
                }),
            )
        },
        |file| {
            return (StatusCode::OK, Json(file.clone()));
        },
    )
}

async fn put_file(Json(payload): Json<PutFile>) -> StatusCode {
    let PutFile { uuid, name, data } = payload;
    log::info!("uuid: {uuid}");
    log::info!("name: {name}");
    log::info!("data: ... (omit)");
    CURRENT_FILE
        .lock()
        .unwrap()
        .borrow_mut()
        .replace(File { uuid, data, name });
    StatusCode::OK
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let endpoint = std::env::var("HERE_ENDPOINT").unwrap_or("0.0.0.0:3000".into());

    let app = Router::new()
        .route("/file", get(get_file))
        .route("/file", post(put_file));

    axum::Server::bind(&endpoint.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
