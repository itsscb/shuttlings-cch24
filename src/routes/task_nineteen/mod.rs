mod cite;
mod db;
mod draft;
mod list;
mod remove;
mod reset;
mod undo;

use std::fmt::{self, Display, Formatter};

pub use cite::cite;
pub use draft::draft;
pub use list::list;
pub use remove::remove;
pub use undo::undo;

pub use reset::reset;
use serde::{Deserialize, Serialize};
use sqlx::types::uuid;

#[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize)]
pub struct Quote {
    id: uuid::Uuid,
    author: String,
    #[allow(clippy::struct_field_names)]
    quote: String,
    created_at: chrono::DateTime<chrono::Utc>,
    version: i32,
}

impl Display for Quote {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string(self).expect("Failed to serialize Quote")
        )
    }
}
