use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;
use sqlx::Error;
use uuid::Uuid;

use crate::{
    auth::middleware::AuthUser,
    models::habit::{CompleteHabit, CreateHabit, Habit, HabitCompletion, UpdateHabit},
    state::AppState,
};

pub async fn get_habits(auth: AuthUser, State(state): State<AppState>) -> impl IntoResponse {
    let habits = sqlx::query_as::<_, Habit>("SELECT * FROM habits WHERE user_id = $1")
        .bind(auth.user_id)
        .fetch_all(&state.db_pool)
        .await;

    match habits {
        Ok(habits) => (StatusCode::OK, Json(json!({"habits": habits}))).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to fetch habits"})),
        )
            .into_response(),
    }
}

pub async fn create_habit(
    auth: AuthUser,
    State(state): State<AppState>,
    Json(body): Json<CreateHabit>,
) -> impl IntoResponse {
    let habit = sqlx::query_as::<_, Habit>(
        "INSERT INTO habits (user_id, name, frequency) \
         VALUES ($1, $2, COALESCE($3, 'daily')) \
         RETURNING *",
    )
    .bind(auth.user_id)
    .bind(body.name)
    .bind(body.frequency)
    .fetch_one(&state.db_pool)
    .await;

    match habit {
        Ok(habit) => (StatusCode::CREATED, Json(json!({"habit": habit}))).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to create habit"})),
        )
            .into_response(),
    }
}

pub async fn update_habit(
    auth: AuthUser,
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
    Json(body): Json<UpdateHabit>,
) -> impl IntoResponse {
    let habit = sqlx::query_as::<_, Habit>(
        "UPDATE habits
         SET name = COALESCE($3, name),
             frequency = COALESCE($4, frequency)
         WHERE id = $1 AND user_id = $2
         RETURNING *",
    )
    .bind(id)
    .bind(auth.user_id)
    .bind(body.name)
    .bind(body.frequency)
    .fetch_optional(&state.db_pool)
    .await;

    match habit {
        Ok(Some(habit)) => (StatusCode::OK, Json(json!({"habit": habit}))).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Habit not found"})),
        )
            .into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to update habit"})),
        )
            .into_response(),
    }
}

pub async fn delete_habit(
    auth: AuthUser,
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let deleted = sqlx::query_scalar::<_, Uuid>(
        "DELETE FROM habits WHERE id = $1 AND user_id = $2 RETURNING id",
    )
    .bind(id)
    .bind(auth.user_id)
    .fetch_optional(&state.db_pool)
    .await;

    match deleted {
        Ok(Some(_)) => StatusCode::NO_CONTENT.into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Habit not found"})),
        )
            .into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to delete habit"})),
        )
            .into_response(),
    }
}

pub async fn complete_habit(
    auth: AuthUser,
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
    Json(body): Json<CompleteHabit>,
) -> impl IntoResponse {
    let completion = sqlx::query_as::<_, HabitCompletion>(
        "INSERT INTO habit_completions (habit_id, completed_on)
         SELECT id, COALESCE($2, CURRENT_DATE)
         FROM habits
         WHERE id = $1 AND user_id = $3
         RETURNING *",
    )
    .bind(id)
    .bind(body.completed_on)
    .bind(auth.user_id)
    .fetch_optional(&state.db_pool)
    .await;

    match completion {
        Ok(Some(completion)) => {
            (StatusCode::OK, Json(json!({"completion": completion}))).into_response()
        }
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Habit not found"})),
        )
            .into_response(),
        Err(Error::Database(db_err)) if db_err.code().as_deref() == Some("23505") => (
            StatusCode::CONFLICT,
            Json(json!({"error": "Habit already completed for that date"})),
        )
            .into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to complete habit"})),
        )
            .into_response(),
    }
}
