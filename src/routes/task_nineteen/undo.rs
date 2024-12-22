use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use tracing::{error, info, instrument};

use super::db;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UndoRequest {
    author: String,
    quote: String,
}

#[instrument(skip(pool))]
#[axum::debug_handler]
pub async fn undo(
    State(pool): State<sqlx::PgPool>,
    Path(id): Path<String>,
    Json(request): Json<UndoRequest>,
) -> impl IntoResponse {
    let id = match uuid::Uuid::parse_str(&id) {
        Ok(id) => id,
        Err(e) => return (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    };
    match db::undo(&pool, id, &request.author, &request.quote)
        .await
        .map_err(|e| {
            error!("Error: {e}");
            (StatusCode::NOT_FOUND, e.to_string())
        }) {
        Ok(quote) => {
            info!("{}", quote.to_string());
            (StatusCode::OK, quote.to_string()).into_response()
        }
        Err((_, message)) => (StatusCode::NOT_FOUND, message).into_response(),
    }
}
