use axum::{
    Router,
    routing::{get, patch, post},
};

use crate::habits::handlers::{
    complete_habit, create_habit, delete_habit, get_habits, update_habit,
};
use crate::state::AppState;

pub fn habits_router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_habits).post(create_habit))
        .route("/{id}", patch(update_habit).delete(delete_habit))
        .route("/{id}/complete", post(complete_habit))
}
