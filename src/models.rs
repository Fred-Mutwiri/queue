use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    pub id: Uuid,
    pub payload: String,
    pub retries: u32,
    pub max_retries: u32,
    pub scheduled_for: DateTime<Utc>, // when job should be executed
}