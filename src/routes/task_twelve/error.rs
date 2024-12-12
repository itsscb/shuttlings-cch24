use std::fmt::Display;

use axum::response::IntoResponse;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameError {
    ColumnFull,
    GameOver,
    InvalidColumn,
    InvalidTeam,
}

impl Display for GameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ColumnFull => write!(f, "Column is full"),
            Self::InvalidColumn => write!(f, "Invalid column"),
            Self::GameOver => write!(f, "Game is over"),
            Self::InvalidTeam => write!(f, "Invalid team"),
        }
    }
}

impl IntoResponse for GameError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::ColumnFull | Self::GameOver | Self::InvalidColumn => (
                axum::http::StatusCode::SERVICE_UNAVAILABLE,
                self.to_string(),
            )
                .into_response(),
            Self::InvalidTeam => {
                (axum::http::StatusCode::BAD_REQUEST, self.to_string()).into_response()
            }
        }
    }
}
