#![allow(dead_code, clippy::unused_async)]

use std::{net::Ipv6Addr, str::FromStr};

use axum::{
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use super::KeyParams;

pub async fn ipv6_key(params: Query<KeyParams>) -> Result<String, Response> {
    let params: KeyParams = params.0;
    let Ok(from) = Ipv6Addr::from_str(&params.from) else {
        return Err((StatusCode::BAD_REQUEST, "Invalid from IP address").into_response());
    };
    let Ok(to) = Ipv6Addr::from_str(&params.to) else {
        return Err((StatusCode::BAD_REQUEST, "Invalid key IP address").into_response());
    };

    Ok(calculate_ipv6_key(from, to))
}

pub fn calculate_ipv6_key(from: Ipv6Addr, to: Ipv6Addr) -> String {
    let result: Vec<String> = to
        .segments()
        .iter()
        .zip(from.segments().iter())
        .map(|(a, b)| a ^ b)
        .map(|s| format!("{s:x}"))
        .collect();

    let ip: Ipv6Addr = result.join(":").parse().unwrap();

    ip.to_string()
}
