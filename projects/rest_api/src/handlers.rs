use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::db::Db;
use crate::models::{CreateItem, UpdateItem};

pub async fn list_items(State(db): State<Db>) -> impl IntoResponse {
    let items = db.list().await;
    Json(items)
}

pub async fn get_item(
    State(db): State<Db>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match db.get(&id).await {
        Some(item) => (StatusCode::OK, Json(item)).into_response(),
        None => (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "not found"}))).into_response(),
    }
}

pub async fn create_item(
    State(db): State<Db>,
    Json(data): Json<CreateItem>,
) -> impl IntoResponse {
    let item = db.create(data).await;
    (StatusCode::CREATED, Json(item))
}

pub async fn update_item(
    State(db): State<Db>,
    Path(id): Path<String>,
    Json(data): Json<UpdateItem>,
) -> impl IntoResponse {
    match db.update(&id, data).await {
        Some(item) => (StatusCode::OK, Json(item)).into_response(),
        None => (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "not found"}))).into_response(),
    }
}

pub async fn delete_item(
    State(db): State<Db>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    if db.delete(&id).await {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}
