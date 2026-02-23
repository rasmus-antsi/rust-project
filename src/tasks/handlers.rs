use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;
use uuid::Uuid;

use crate::{
    auth::middleware::AuthUser,
    models::task::{CreateTask, Task, UpdateTask},
    state::AppState,
};

pub async fn get_tasks(auth: AuthUser, State(state): State<AppState>) -> impl IntoResponse {
    let tasks = sqlx::query_as::<_, Task>("SELECT * FROM tasks WHERE user_id = $1")
        .bind(auth.user_id)
        .fetch_all(&state.db_pool)
        .await;

    match tasks {
        Ok(tasks) => (StatusCode::OK, Json(json!({"tasks": tasks}))).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to fetch tasks"})),
        )
            .into_response(),
    }
}

pub async fn create_task(
    auth: AuthUser,
    State(state): State<AppState>,
    Json(body): Json<CreateTask>,
) -> impl IntoResponse {
    let task = sqlx::query_as::<_, Task>(
        "INSERT INTO tasks (user_id, title, notes, priority, due_date) \
         VALUES ($1, $2, $3, COALESCE($4, 'medium'), $5) \
         RETURNING *",
    )
    .bind(auth.user_id)
    .bind(body.title)
    .bind(body.notes)
    .bind(body.priority)
    .bind(body.due_date)
    .fetch_one(&state.db_pool)
    .await;

    match task {
        Ok(task) => (StatusCode::CREATED, Json(json!({"task": task}))).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to create task"})),
        )
            .into_response(),
    }
}

pub async fn update_task(
    auth: AuthUser,
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
    Json(body): Json<UpdateTask>,
) -> impl IntoResponse {
    let task = sqlx::query_as::<_, Task>(
        "UPDATE tasks
         SET title = COALESCE($3, title),
             notes = COALESCE($4, notes),
             priority = COALESCE($5, priority),
             due_date = COALESCE($6, due_date),
             completed = COALESCE($7, completed),
             completed_at = CASE
                 WHEN COALESCE($7, completed) = TRUE AND completed = FALSE THEN NOW()
                 WHEN COALESCE($7, completed) = FALSE THEN NULL
                 ELSE completed_at
             END
         WHERE id = $1 AND user_id = $2
         RETURNING *",
    )
    .bind(id)
    .bind(auth.user_id)
    .bind(body.title)
    .bind(body.notes)
    .bind(body.priority)
    .bind(body.due_date)
    .bind(body.completed)
    .fetch_optional(&state.db_pool)
    .await;

    match task {
        Ok(Some(task)) => (StatusCode::OK, Json(json!({"task": task}))).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Task not found"})),
        )
            .into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to update task"})),
        )
            .into_response(),
    }
}

pub async fn delete_task(
    auth: AuthUser,
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let deleted = sqlx::query_scalar::<_, Uuid>(
        "DELETE FROM tasks WHERE id = $1 AND user_id = $2 RETURNING id",
    )
    .bind(id)
    .bind(auth.user_id)
    .fetch_optional(&state.db_pool)
    .await;

    match deleted {
        Ok(Some(_)) => StatusCode::NO_CONTENT.into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Task not found"})),
        )
            .into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to delete task"})),
        )
            .into_response(),
    }
}

pub async fn complete_task(
    auth: AuthUser,
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let task = sqlx::query_as::<_, Task>(
        "UPDATE tasks
         SET completed = TRUE, completed_at = NOW()
         WHERE id = $1 AND user_id = $2
         RETURNING *",
    )
    .bind(id)
    .bind(auth.user_id)
    .fetch_optional(&state.db_pool)
    .await;

    match task {
        Ok(Some(task)) => (StatusCode::OK, Json(json!({"task": task}))).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Task not found"})),
        )
            .into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to complete task"})),
        )
            .into_response(),
    }
}
