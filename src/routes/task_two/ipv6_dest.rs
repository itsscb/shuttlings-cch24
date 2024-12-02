#![allow(dead_code, clippy::unused_async)]

use std::{net::Ipv6Addr, str::FromStr};

use axum::{
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use super::DestParams;

pub async fn ipv6_dest(params: Query<DestParams>) -> Result<String, Response> {
    let params: DestParams = params.0;
    let Ok(from) = Ipv6Addr::from_str(&params.from) else {
        return Err((StatusCode::BAD_REQUEST, "Invalid from IP address").into_response());
    };
    let Ok(key) = Ipv6Addr::from_str(&params.key) else {
        return Err((StatusCode::BAD_REQUEST, "Invalid key IP address").into_response());
    };

    Ok(calculate_ipv6_dest(from, key))
}

pub fn calculate_ipv6_dest(from: Ipv6Addr, key: Ipv6Addr) -> String {
    let result: Vec<String> = from
        .segments()
        .iter()
        .zip(key.segments().iter())
        .map(|(a, b)| a ^ b)
        .map(|s| format!("{s:x}"))
        .collect();

    let ip: Ipv6Addr = result.join(":").parse().unwrap();

    ip.to_string()
}
