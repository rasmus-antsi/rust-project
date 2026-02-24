use axum::{
    Router,
    routing::{get, patch, post},
};

use crate::goals::handlers::{complete_goal, create_goal, delete_goal, get_goals, update_goal};
use crate::state::AppState;

pub fn goals_router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_goals).post(create_goal))
        .route("/{id}", patch(update_goal).delete(delete_goal))
        .route("/{id}/complete", post(complete_goal))
}
