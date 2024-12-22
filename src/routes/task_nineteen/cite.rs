use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use tracing::{error, instrument, trace};

use super::db;

#[instrument(skip(pool))]
#[axum::debug_handler]
pub async fn cite(State(pool): State<sqlx::PgPool>, Path(id): Path<String>) -> impl IntoResponse {
    let id = match uuid::Uuid::parse_str(&id) {
        Ok(id) => id,
        Err(e) => return (StatusCode::NOT_FOUND, e.to_string()).into_response(),
    };
    match db::get(&pool, id).await.map_err(|e| {
        error!("Error: {e}");
        (StatusCode::NOT_FOUND, e.to_string())
    }) {
        Ok(quote) => {
            trace!("{}", quote.to_string());
            (StatusCode::OK, quote.to_string()).into_response()
        }
        Err((_, message)) => (StatusCode::NOT_FOUND, message).into_response(),
    }
}
