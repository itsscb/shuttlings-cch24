#![allow(dead_code, clippy::unused_async)]

use axum::{
    http::{header, StatusCode},
    response::{AppendHeaders, IntoResponse},
};
pub async fn minus_one() -> impl IntoResponse {
    (
        StatusCode::FOUND,
        AppendHeaders([(
            header::LOCATION,
            "https://www.youtube.com/watch?v=9Gc4QTqslN4",
        )]),
    )
}
