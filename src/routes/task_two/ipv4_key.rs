#![allow(dead_code, clippy::unused_async)]

use std::{net::Ipv4Addr, str::FromStr};

use axum::{
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use super::KeyParams;

pub async fn ipv4_key(params: Query<KeyParams>) -> Result<String, Response> {
    let params: KeyParams = params.0;
    let Ok(from) = Ipv4Addr::from_str(&params.from) else {
        return Err((StatusCode::BAD_REQUEST, "Invalid from IP address").into_response());
    };
    let Ok(to) = Ipv4Addr::from_str(&params.to) else {
        return Err((StatusCode::BAD_REQUEST, "Invalid key IP address").into_response());
    };
    Ok(calculate_ipv4_key(from, to))
}

pub fn calculate_ipv4_key(from: Ipv4Addr, to: Ipv4Addr) -> String {
    let result: Vec<u8> = to
        .octets()
        .iter()
        .zip(from.octets().iter())
        .map(|(a, b)| a.overflowing_sub(*b).0)
        .collect();
    result
        .iter()
        .map(std::string::ToString::to_string)
        .collect::<Vec<String>>()
        .join(".")
}
