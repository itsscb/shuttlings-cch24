use axum::{
    http::{header::CONTENT_TYPE, HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::{json, Value};
use tracing::{info, instrument, warn};

use super::claims::Claims;

#[instrument(skip(headers, jar))]
pub async fn wrap(
    headers: HeaderMap,
    jar: CookieJar,
    Json(payload): Json<Value>,
) -> Result<CookieJar, Response> {
    match headers.get(CONTENT_TYPE) {
        Some(content_type) => match content_type.to_str().unwrap() {
            "application/json" => (),
            _ => {
                return Err((StatusCode::UNSUPPORTED_MEDIA_TYPE, "").into_response());
            }
        },
        None => {
            return Err((
                StatusCode::UNSUPPORTED_MEDIA_TYPE,
                "Content type not provided",
            )
                .into_response());
        }
    }

    let payload_str = payload.to_string();
    warn!("{}", json!(&payload).to_string());
    info!("payload" = payload_str);
    let token = encode(
        &Header::default(),
        &Claims::new(Some(payload_str)),
        &EncodingKey::from_secret("secret".as_ref()),
    )
    .map_err(|e| {
        eprintln!("Error: {e}");
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
    })?;

    info!("token" = token);

    let jar = jar.add(Cookie::new("gift", token));

    Ok(jar)
}
