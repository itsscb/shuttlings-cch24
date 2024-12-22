use axum::{extract::State, http::StatusCode, response::IntoResponse};
use tracing::{error, instrument};

use super::db;

#[instrument(skip(pool))]
pub async fn reset(State(pool): State<sqlx::PgPool>) -> impl IntoResponse {
    db::reset_db(&pool)
        .await
        .map_err(|e| {
            error!("Error: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })
        .unwrap();
    StatusCode::OK
}
