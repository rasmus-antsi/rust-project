use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::Type, PartialEq, Clone)]
#[sqlx(type_name = "text", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum GoalStatus {
    Active,
    Completed,
    Abandoned,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Goal {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub deadline: Option<NaiveDate>,
    pub status: GoalStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateGoal {
    pub title: String,
    pub description: Option<String>,
    pub deadline: Option<NaiveDate>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateGoal {
    pub title: Option<String>,
    pub description: Option<String>,
    pub deadline: Option<NaiveDate>,
    pub status: Option<GoalStatus>,
}
