use axum::{
    http::{header::CONTENT_TYPE, HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use cargo_manifest::{Manifest, Package};
use toml::Value;

use super::Orders;

use serde_json::Value as JsonValue;
use serde_yml::Value as YamlValue;

pub fn convert_json_to_toml(json: &str) -> Result<String, Box<dyn std::error::Error>> {
    let value: JsonValue = serde_json::from_str(json)?;
    let toml = toml::to_string(&value)?;
    Ok(toml)
}

pub fn convert_yaml_to_toml(yaml: &str) -> Result<String, Box<dyn std::error::Error>> {
    let value: YamlValue = serde_yml::from_str(yaml)?;
    let toml = toml::to_string(&value)?;
    Ok(toml)
}

pub async fn manifest(headers: HeaderMap, payload: String) -> Result<String, Response> {
    let payload: String = match headers.get(CONTENT_TYPE) {
        Some(content_type) => match content_type.to_str().unwrap() {
            "application/toml" => payload,
            "application/json" => match convert_json_to_toml(&payload) {
                Err(_) => return Err((StatusCode::UNSUPPORTED_MEDIA_TYPE, "").into_response()),
                Ok(toml) => toml,
            },
            "application/yaml" => match convert_yaml_to_toml(&payload) {
                Err(_) => return Err((StatusCode::UNSUPPORTED_MEDIA_TYPE, "").into_response()),
                Ok(toml) => toml,
            },

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
    };

    let orders: Orders = match Manifest::from_slice_with_metadata(payload.as_bytes()) {
        Ok(manifest) => {
            let package: Package = match manifest.package {
                Some(package) => package,
                None => {
                    return Err(
                        (StatusCode::BAD_REQUEST, "Invalid manifest: package").into_response()
                    );
                }
            };

            if let Some(cargo_manifest::MaybeInherited::Local(keywords)) = package.keywords {
                if !keywords.iter().any(|k| k == "Christmas 2024") {
                    return Err(
                        (StatusCode::BAD_REQUEST, "Magic keyword not provided").into_response()
                    );
                }
            } else {
                return Err((StatusCode::BAD_REQUEST, "Magic keyword not provided").into_response());
            }

            let metadata: Value = match package.metadata {
                Some(metadata) => metadata,
                None => {
                    return Err((StatusCode::NO_CONTENT, "").into_response());
                }
            };

            Orders::from(
                metadata
                    .get("orders")
                    .and_then(Value::as_array)
                    .ok_or_else(|| (StatusCode::NO_CONTENT, "orders").into_response())?,
            )
        }
        Err(_) => {
            return Err((StatusCode::BAD_REQUEST, "Invalid manifest").into_response());
        }
    };

    if orders.0.is_empty() {
        return Err((StatusCode::NO_CONTENT, "").into_response());
    }
    Ok(orders.to_string())
}
