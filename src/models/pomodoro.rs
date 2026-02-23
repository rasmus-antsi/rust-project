use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::Type, PartialEq, Clone)]
#[sqlx(type_name = "text", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum SessionType {
    Focus,
    ShortBreak,
    LongBreak,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct PomodoroSession {
    pub id: Uuid,
    pub user_id: Uuid,
    pub task_id: Option<Uuid>,
    pub session_type: SessionType,
    pub duration_minutes: i32,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct StartSession {
    pub task_id: Option<Uuid>,
    pub session_type: Option<SessionType>,
    pub duration_minutes: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct EndSession {
    pub notes: Option<String>,
}
