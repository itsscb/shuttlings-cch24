use std::fmt::Display;

use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse},
};
use html_escape::encode_safe;
use tracing::{info, instrument};

#[derive(Debug, PartialEq, Clone)]
enum State {
    On,
    Off,
}

impl State {
    const fn next(&self) -> Self {
        match self {
            Self::On => Self::Off,
            Self::Off => Self::On,
        }
    }

    const fn is_on(&self) -> &str {
        match self {
            Self::On => " on",
            Self::Off => "",
        }
    }
}

impl TryFrom<String> for State {
    type Error = &'static str;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "on" => Ok(Self::On),
            "off" => Ok(Self::Off),
            _ => Err("invalid state"),
        }
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state = match self {
            Self::On => "on",
            Self::Off => "off",
        };
        write!(f, "{state}")
    }
}

#[instrument()]
pub async fn ornament(Path((state, id)): Path<(String, String)>) -> impl IntoResponse {
    State::try_from(state).map_or_else(|_| (StatusCode::IM_A_TEAPOT, "I'm a teapot").into_response(),
        |state| {
            let ornament = format!(r#"<div class="ornament{on}" id="ornament{id}" hx-trigger="load delay:2s once" hx-get="/23/ornament/{next_state}/{id}" hx-swap="outerHTML"></div>"#,on = state.is_on(), id = encode_safe(&id), next_state = state.next() );
            info!(ornament);
            Html(ornament).into_response()
        }
    )
}
