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
    models::pomodoro::{EndSession, PomodoroSession, StartSession},
    state::AppState,
};

pub async fn get_sessions(auth: AuthUser, State(state): State<AppState>) -> impl IntoResponse {
    let sessions = sqlx::query_as::<_, PomodoroSession>(
        "SELECT * FROM pomodoro_sessions WHERE user_id = $1 ORDER BY started_at DESC",
    )
    .bind(auth.user_id)
    .fetch_all(&state.db_pool)
    .await;

    match sessions {
        Ok(sessions) => (StatusCode::OK, Json(json!({"sessions": sessions}))).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to fetch pomodoro sessions"})),
        )
            .into_response(),
    }
}

pub async fn start_session(
    auth: AuthUser,
    State(state): State<AppState>,
    Json(body): Json<StartSession>,
) -> impl IntoResponse {
    let session = sqlx::query_as::<_, PomodoroSession>(
        "INSERT INTO pomodoro_sessions (user_id, task_id, session_type, duration_minutes)
         SELECT
             $1,
             CASE WHEN $2::uuid IS NULL THEN NULL ELSE t.id END,
             COALESCE($3, 'focus'),
             COALESCE($4, 25)
         FROM (SELECT 1) seed
         LEFT JOIN tasks t ON t.id = $2 AND t.user_id = $1
         WHERE $2::uuid IS NULL OR t.id IS NOT NULL
         RETURNING *",
    )
    .bind(auth.user_id)
    .bind(body.task_id)
    .bind(body.session_type)
    .bind(body.duration_minutes)
    .fetch_optional(&state.db_pool)
    .await;

    match session {
        Ok(Some(session)) => {
            (StatusCode::CREATED, Json(json!({"session": session}))).into_response()
        }
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Task not found"})),
        )
            .into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to start pomodoro session"})),
        )
            .into_response(),
    }
}

pub async fn end_session(
    auth: AuthUser,
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
    Json(body): Json<EndSession>,
) -> impl IntoResponse {
    let session = sqlx::query_as::<_, PomodoroSession>(
        "UPDATE pomodoro_sessions
         SET ended_at = COALESCE(ended_at, NOW()),
             notes = COALESCE($3, notes)
         WHERE id = $1 AND user_id = $2
         RETURNING *",
    )
    .bind(id)
    .bind(auth.user_id)
    .bind(body.notes)
    .fetch_optional(&state.db_pool)
    .await;

    match session {
        Ok(Some(session)) => (StatusCode::OK, Json(json!({"session": session}))).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Session not found"})),
        )
            .into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to end pomodoro session"})),
        )
            .into_response(),
    }
}

pub async fn delete_session(
    auth: AuthUser,
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let deleted = sqlx::query_scalar::<_, Uuid>(
        "DELETE FROM pomodoro_sessions WHERE id = $1 AND user_id = $2 RETURNING id",
    )
    .bind(id)
    .bind(auth.user_id)
    .fetch_optional(&state.db_pool)
    .await;

    match deleted {
        Ok(Some(_)) => StatusCode::NO_CONTENT.into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Session not found"})),
        )
            .into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to delete pomodoro session"})),
        )
            .into_response(),
    }
}
