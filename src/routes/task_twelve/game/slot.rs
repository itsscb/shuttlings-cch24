use std::fmt::Display;

use serde::Deserialize;

use super::{super::error::GameError, COOKIE, MILK};

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Slot {
    Milk,
    Cookie,
}

impl Display for Slot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Milk => write!(f, "{MILK}"),
            Self::Cookie => write!(f, "{COOKIE}"),
        }
    }
}

impl TryFrom<&str> for Slot {
    type Error = GameError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "milk" => Ok(Self::Milk),
            "cookie" => Ok(Self::Cookie),
            _ => Err(GameError::InvalidTeam),
        }
    }
}
