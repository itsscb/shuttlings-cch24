use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use axum_extra::extract::CookieJar;
use jsonwebtoken::{DecodingKey, Validation};
use tracing::{info, instrument};

use super::claims::Claims;

#[instrument(skip(jar))]
pub async fn unwrap(jar: CookieJar) -> Result<String, Response> {
    let payload = jar
        .get("gift")
        .map(|cookie| cookie.value().to_string())
        .map(|token| {
            let token = jsonwebtoken::decode::<Claims>(
                &token,
                &DecodingKey::from_secret("secret".as_ref()),
                &Validation::default(),
            );
            token
        });

    match payload {
        Some(Ok(claims)) => {
            info!("claims" = claims.claims.sub());
            claims
                .claims
                .sub()
                .map_or_else(|| Err((StatusCode::BAD_REQUEST, "").into_response()), Ok)
        }
        _ => Err((StatusCode::BAD_REQUEST, "").into_response()),
    }
}
