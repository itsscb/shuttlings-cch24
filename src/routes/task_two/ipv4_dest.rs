#![allow(dead_code, clippy::unused_async)]

use std::{net::Ipv4Addr, str::FromStr};

use axum::{
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use super::DestParams;

pub async fn ipv4_dest(params: Query<DestParams>) -> Result<String, Response> {
    let params: DestParams = params.0;
    let Ok(from) = Ipv4Addr::from_str(&params.from) else {
        return Err((StatusCode::BAD_REQUEST, "Invalid from IP address").into_response());
    };
    let Ok(key) = Ipv4Addr::from_str(&params.key) else {
        return Err((StatusCode::BAD_REQUEST, "Invalid key IP address").into_response());
    };

    Ok(calculate_ipv4_dest(from, key))
}

pub fn calculate_ipv4_dest(from: Ipv4Addr, key: Ipv4Addr) -> String {
    let result: Vec<u8> = from
        .octets()
        .iter()
        .zip(key.octets().iter())
        .map(|(a, b)| a.overflowing_add(*b).0)
        .collect();
    result
        .iter()
        .map(std::string::ToString::to_string)
        .collect::<Vec<String>>()
        .join(".")
}
