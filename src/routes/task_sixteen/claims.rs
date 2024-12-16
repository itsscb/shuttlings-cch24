use std::fmt::{self, Display, Formatter};

use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    exp: Option<i64>,
    #[serde(rename = "sub")]
    sub: Option<String>,
}

impl Claims {
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    pub fn new(sub: Option<String>) -> Self {
        let exp = Some((Utc::now() + Duration::minutes(15)).timestamp());
        Self { exp, sub }
    }

    pub fn sub(&self) -> Option<String> {
        self.sub.clone()
    }
}

impl Display for Claims {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.exp {
            Some(exp) => write!(
                f,
                "expires: {:?}\nsub: {}",
                exp,
                self.sub.as_deref().unwrap_or("None")
            ),
            None => write!(
                f,
                "expires: None\nsub: {}",
                self.sub.as_deref().unwrap_or("None")
            ),
        }
    }
}
