use std::fmt::Display;

use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse},
};
use tracing::instrument;

#[derive(Debug, PartialEq, Clone)]
enum Color {
    Red,
    Blue,
    Purple,
}

impl Color {
    const fn next(&self) -> Self {
        match self {
            Self::Red => Self::Blue,
            Self::Blue => Self::Purple,
            Self::Purple => Self::Red,
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let color = match self {
            Self::Red => "red",
            Self::Blue => "blue",
            Self::Purple => "purple",
        };
        write!(f, "{color}")
    }
}

impl TryFrom<String> for Color {
    type Error = &'static str;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "blue" => Ok(Self::Blue),
            "purple" => Ok(Self::Purple),
            "red" => Ok(Self::Red),
            _ => Err("invalid color"),
        }
    }
}

// impl From<String> for Color {
//     fn from(s: String) -> Self {
//         match s.to_lowercase().as_str() {
//             "blue" => Self::Blue,
//             "purple" => Self::Purple,
//             _ => Self::Red,
//         }
//     }
// }

#[instrument()]
pub async fn present(Path(color): Path<String>) -> impl IntoResponse {
    Color::try_from(color).map_or_else(
        |_| (StatusCode::IM_A_TEAPOT, "I'm a teapot").into_response(),
        |color| {
            Html(format!(
                r#"<div class="present {}" hx-get="/23/present/{}" hx-swap="outerHTML">
        <div class="ribbon"></div>
        <div class="ribbon"></div>
        <div class="ribbon"></div>
        <div class="ribbon"></div>
    </div>"#,
                color,
                color.next()
            ))
            .into_response()
        },
    )
}
