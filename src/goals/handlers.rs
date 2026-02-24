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
    models::goal::{CreateGoal, Goal, UpdateGoal},
    state::AppState,
};

pub async fn get_goals(auth: AuthUser, State(state): State<AppState>) -> impl IntoResponse {
    let goals = sqlx::query_as::<_, Goal>("SELECT * FROM goals WHERE user_id = $1")
        .bind(auth.user_id)
        .fetch_all(&state.db_pool)
        .await;

    match goals {
        Ok(goals) => (StatusCode::OK, Json(json!({"goals": goals}))).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to fetch goals"})),
        )
            .into_response(),
    }
}

pub async fn create_goal(
    auth: AuthUser,
    State(state): State<AppState>,
    Json(body): Json<CreateGoal>,
) -> impl IntoResponse {
    let goal = sqlx::query_as::<_, Goal>(
        "INSERT INTO goals (user_id, title, description, deadline, status) \
         VALUES ($1, $2, $3, $4, 'active') \
         RETURNING *",
    )
    .bind(auth.user_id)
    .bind(body.title)
    .bind(body.description)
    .bind(body.deadline)
    .fetch_one(&state.db_pool)
    .await;

    match goal {
        Ok(goal) => (StatusCode::CREATED, Json(json!({"goal": goal}))).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to create goal"})),
        )
            .into_response(),
    }
}

pub async fn update_goal(
    auth: AuthUser,
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
    Json(body): Json<UpdateGoal>,
) -> impl IntoResponse {
    let goal = sqlx::query_as::<_, Goal>(
        "UPDATE goals
         SET title = COALESCE($3, title),
             description = COALESCE($4, description),
             deadline = COALESCE($5, deadline),
             status = COALESCE($6, status)
         WHERE id = $1 AND user_id = $2
         RETURNING *",
    )
    .bind(id)
    .bind(auth.user_id)
    .bind(body.title)
    .bind(body.description)
    .bind(body.deadline)
    .bind(body.status)
    .fetch_optional(&state.db_pool)
    .await;

    match goal {
        Ok(Some(goal)) => (StatusCode::OK, Json(json!({"goal": goal}))).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Goal not found"})),
        )
            .into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to update goal"})),
        )
            .into_response(),
    }
}

pub async fn delete_goal(
    auth: AuthUser,
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let deleted = sqlx::query_scalar::<_, Uuid>(
        "DELETE FROM goals WHERE id = $1 AND user_id = $2 RETURNING id",
    )
    .bind(id)
    .bind(auth.user_id)
    .fetch_optional(&state.db_pool)
    .await;

    match deleted {
        Ok(Some(_)) => StatusCode::NO_CONTENT.into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Goal not found"})),
        )
            .into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to delete goal"})),
        )
            .into_response(),
    }
}

pub async fn complete_goal(
    auth: AuthUser,
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let goal = sqlx::query_as::<_, Goal>(
        "UPDATE goals
         SET status = 'completed'
         WHERE id = $1 AND user_id = $2
         RETURNING *",
    )
    .bind(id)
    .bind(auth.user_id)
    .fetch_optional(&state.db_pool)
    .await;

    match goal {
        Ok(Some(goal)) => (StatusCode::OK, Json(json!({"goal": goal}))).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Goal not found"})),
        )
            .into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to complete goal"})),
        )
            .into_response(),
    }
}
