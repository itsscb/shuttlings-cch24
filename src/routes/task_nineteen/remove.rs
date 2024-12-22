use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use tracing::{error, info, instrument};

use super::db;

#[instrument(skip(pool))]
#[axum::debug_handler]
pub async fn remove(State(pool): State<sqlx::PgPool>, Path(id): Path<String>) -> impl IntoResponse {
    let id = match uuid::Uuid::parse_str(&id) {
        Ok(id) => id,
        Err(e) => return (StatusCode::NOT_FOUND, e.to_string()).into_response(),
    };
    match db::remove(&pool, id).await.map_err(|e| {
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
