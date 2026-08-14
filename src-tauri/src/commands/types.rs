use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TaskPriority {
    Low,
    Medium,
    High,
}

impl TaskPriority {
    pub fn as_i32(&self) -> i32 {
        match self {
            TaskPriority::Low => 0,
            TaskPriority::Medium => 1,
            TaskPriority::High => 2,
        }
    }
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(TaskPriority::Low),
            1 => Some(TaskPriority::Medium),
            2 => Some(TaskPriority::High),
            _ => None,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: i32,
    pub name: String,
    pub due_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub completed: bool,
    pub completed_at: Option<DateTime<Utc>>,
    pub priority: Option<TaskPriority>,
    pub tags: Option<Vec<Tag>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub id: i64,
    pub name: String,
    pub color: String,
}

#[derive(Serialize, Deserialize)]
pub struct NewTag {
    pub name: String,
    pub color: String,
}
