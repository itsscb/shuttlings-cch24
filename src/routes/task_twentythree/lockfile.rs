use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};
use axum_extra::extract::Multipart;
use regex::Regex;
use toml::Value;
use tracing::{error, info, instrument};

#[instrument()]
pub async fn lockfile(
    mut multipart: Multipart,
) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    let mut content = String::new();
    while let Some(field) = multipart.next_field().await.map_err(|_| {
        error!("Failed to read field from multipart");
        (
            StatusCode::BAD_REQUEST,
            "Failed to read field from multipart",
        )
    })? {
        let data = field.bytes().await.map_err(|_| {
            error!("Failed to read bytes from multipart");
            (
                StatusCode::BAD_REQUEST,
                "Failed to read bytes from multipart",
            )
        })?;

        content.push_str(&String::from_utf8_lossy(&data));
    }

    if toml::from_str::<Value>(&content).is_err() {
        error!("Failed to parse TOML from multipart");
        return Err((
            StatusCode::BAD_REQUEST,
            "Failed to parse TOML from multipart",
        ));
    }

    let re = Regex::new(r#"checksum = "([^"]+)"\n"#)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Failed to compile regex"))?;

    let mut output = Vec::new();
    for cap in re.captures_iter(&content) {
        if let Some(m) = cap.get(1) {
            let checksum = m.as_str();
            match checksum_to_sprinkle(checksum) {
                Ok(sprinkle) => output.push(sprinkle),
                Err(_) => return Err((StatusCode::UNPROCESSABLE_ENTITY, "Invalid checksum")),
            }
        }
    }

    if output.is_empty() {
        error!("No checksums found");
        return Err((StatusCode::BAD_REQUEST, "No checksums found"));
    }

    info!(output = ?output);
    Ok(Html(output.join("\n")).into_response())
}

fn checksum_to_sprinkle(checksum: &str) -> Result<String, &'static str> {
    if hex::decode(checksum).is_err() {
        return Err("Invalid checksum");
    }

    let color = checksum.chars().take(6).collect::<String>();
    let top = u32::from_str_radix(&checksum.chars().skip(6).take(2).collect::<String>(), 16)
        .map_err(|_| "Invalid top value")?;

    let left = u32::from_str_radix(&checksum.chars().skip(8).take(2).collect::<String>(), 16)
        .map_err(|_| "Invalid top value")?;

    Ok(format!(
        r#"<div style="background-color:#{color};top:{top}px;left:{left}px;"></div>"#
    ))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_checksum_to_sprinkle() {
        assert_eq!(
            checksum_to_sprinkle(
                "337789faa0372648a8ac286b2f92a53121fe118f12e29009ac504872a5413cc6"
            ),
            Ok(r#"<div style="background-color:#337789;top:250px;left:160px;"></div>"#.to_string())
        );
        assert_eq!(
            checksum_to_sprinkle(
                "22ba454b13e4e29b5b892a62c334360a571de5a25c936283416c94328427dd57"
            ),
            Ok(r#"<div style="background-color:#22ba45;top:75px;left:19px;"></div>"#.to_string())
        );
    }
}
