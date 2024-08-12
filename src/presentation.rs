use crate::service::{DeleteResult, PasteService, PasteServiceError};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

type PasteState = State<Arc<Box<dyn PasteService>>>;

pub async fn listen(paste_service: Box<dyn PasteService>) {
    let app = Router::new()
        .route("/p", post(create_paste))
        .route("/p/:slug", get(get_paste).delete(delete_paste))
        .with_state(Arc::new(paste_service));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap()
}

async fn get_paste(
    State(paste_service): PasteState,
    Path(slug): Path<String>,
) -> impl IntoResponse {
    match paste_service.get_paste(&slug).await {
        Ok(Some(content)) => (StatusCode::OK, content).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(err) => err.into_response(),
    }
}

#[derive(Debug, Deserialize)]
struct CreatePasteInput {
    pub content: String,
}

async fn create_paste(
    State(paste_service): PasteState,
    Json(input): Json<CreatePasteInput>,
) -> impl IntoResponse {
    match paste_service.create_paste(input.content).await {
        Ok(slug) => (StatusCode::OK, slug).into_response(),
        Err(err) => err.into_response(),
    }
}

async fn delete_paste(
    State(paste_service): PasteState,
    Path(slug): Path<String>,
) -> impl IntoResponse {
    match paste_service.delete_paste(&slug).await {
        Ok(DeleteResult::Present) => StatusCode::OK.into_response(),
        Ok(DeleteResult::NotPresent) => StatusCode::NOT_FOUND.into_response(),
        Err(err) => err.into_response(),
    }
}

impl IntoResponse for PasteServiceError {
    fn into_response(self) -> Response {
        #[derive(Serialize)]
        struct ErrorResponse {
            message: String,
        }
        let status_code = match self {
            PasteServiceError::ContentTooLarge => StatusCode::BAD_REQUEST,
            PasteServiceError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            PasteServiceError::InvalidSlug => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let response_json = serde_json::to_string(&ErrorResponse {
            message: self.to_string(),
        })
        .unwrap();

        (status_code, response_json).into_response()
    }
}
