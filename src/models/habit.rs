use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::Type, PartialEq, Clone)]
#[sqlx(type_name = "text", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Frequency {
    Daily,
    Weekly,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Habit {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub frequency: Frequency,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct HabitCompletion {
    pub id: Uuid,
    pub habit_id: Uuid,
    pub completed_on: NaiveDate,
}

#[derive(Debug, Deserialize)]
pub struct CreateHabit {
    pub name: String,
    pub frequency: Option<Frequency>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateHabit {
    pub name: Option<String>,
    pub frequency: Option<Frequency>,
}

#[derive(Debug, Deserialize)]
pub struct CompleteHabit {
    pub completed_on: Option<NaiveDate>,
}
