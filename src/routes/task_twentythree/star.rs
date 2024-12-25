use axum::response::{Html, IntoResponse};
use tracing::instrument;

#[instrument()]
pub async fn star() -> impl IntoResponse {
    Html(r#"<div id="star" class="lit"></div>"#)
}
