use axum::{
    extract::State,
    http::{header::CONTENT_TYPE, HeaderMap, StatusCode},
    response::IntoResponse,
};

use super::{MilkFactory, Unit};

#[allow(clippy::unused_async)]
pub async fn refill(State(milk_factory): State<MilkFactory>) -> impl IntoResponse {
    milk_factory.magic_refill();
    String::new().into_response()
}

#[allow(clippy::unused_async)]
pub async fn milk(
    State(milk_factory): State<MilkFactory>,
    headers: HeaderMap,
    payload: String,
) -> Result<String, impl IntoResponse> {
    headers.get(CONTENT_TYPE).map_or_else(
        || match milk_factory.withdraw() {
            Ok(message) => Ok(message.to_string()),
            Err(message) => Err(message.into_response()),
        },
        |content_type| match content_type.to_str().unwrap() {
            "application/json" => Unit::from_json(&payload).map_or_else(
                |_| {
                    let _ = milk_factory.withdraw();
                    Err((StatusCode::BAD_REQUEST, "").into_response())
                },
                |unit| match unit {
                    Unit::Liters(_) | Unit::Gallons(_) | Unit::Litres(_) | Unit::Pints(_) => {
                        match milk_factory.withdraw() {
                            Ok(_) => Ok(unit.json().unwrap()),
                            Err(message) => Err(message.into_response()),
                        }
                    }
                },
            ),
            _ => match milk_factory.withdraw() {
                Ok(message) => Ok(message.to_string()),
                Err(message) => Err(message.into_response()),
            },
        },
    )
}
