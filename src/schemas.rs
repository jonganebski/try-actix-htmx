use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    description: String,
    done: bool,
    id: i32,
}
