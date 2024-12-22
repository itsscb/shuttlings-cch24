use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use tracing::{error, info, instrument};

use super::db;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DraftRequest {
    author: String,
    quote: String,
}

#[instrument(skip(pool))]
pub async fn draft(
    State(pool): State<sqlx::PgPool>,
    Json(request): Json<DraftRequest>,
) -> impl IntoResponse {
    match db::draft(&pool, &request.author, &request.quote)
        .await
        .map_err(|e| {
            error!("Error: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        }) {
        Ok(quote) => {
            info!("{}", quote.to_string());
            (StatusCode::CREATED, quote.to_string()).into_response()
        }
        Err((status, message)) => (status, message).into_response(),
    }
}
