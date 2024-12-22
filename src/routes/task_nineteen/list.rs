use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
    str::FromStr,
};

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{de, Deserialize, Deserializer, Serialize};
use tracing::{error, info, instrument};

use super::{db, Quote};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ListResponse {
    page: u32,
    next_token: Option<String>,
    quotes: Vec<Quote>,
}

impl ListResponse {
    pub fn new(page: u32, next_page: Option<u32>, quotes: Vec<Quote>) -> Self {
        let next_token = next_page.map(|page| format!("{page:0>16}"));
        Self {
            page,
            next_token,
            quotes,
        }
    }
}

impl Display for ListResponse {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string(self).expect("Failed to serialize ListResponse")
        )
    }
}

fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: fmt::Display,
{
    let opt = Option::<String>::deserialize(de)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => FromStr::from_str(s).map_err(de::Error::custom).map(Some),
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token(#[serde(default, deserialize_with = "empty_string_as_none")] Option<String>);

#[instrument(skip(pool))]
#[axum::debug_handler]
pub async fn list(
    State(pool): State<sqlx::PgPool>,
    Query(token): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let next_page = match token.get("token") {
        Some(token) => {
            let t = token
                .chars()
                .last()
                .and_then(|c| c.to_string().parse::<u32>().ok());

            if t.is_some() {
                t
            } else {
                return (StatusCode::BAD_REQUEST, "Invalid token".to_string()).into_response();
            }
        }
        None => None,
    };

    match db::list(&pool, next_page).await.map_err(|e| {
        error!("Error: {e}");
        (StatusCode::NOT_FOUND, e.to_string())
    }) {
        Ok((quotes, page, next)) => {
            let resp = ListResponse::new(page, next, quotes);
            info!("{}", resp.to_string()); // Changed from error to info
            (StatusCode::OK, resp.to_string()).into_response()
        }
        Err((code, message)) => {
            error!("{}: {}", code, message);
            (code, message).into_response()
        }
    }
}
