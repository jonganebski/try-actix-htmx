use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub description: String,
    pub done: bool,
    pub id: i32,
}
