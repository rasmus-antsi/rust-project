use axum::{
    Router,
    routing::{get, patch, post},
};

use crate::state::AppState;
use crate::tasks::handlers::{complete_task, create_task, delete_task, get_tasks, update_task};

pub fn tasks_router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_tasks).post(create_task))
        .route("/{id}", patch(update_task).delete(delete_task))
        .route("/{id}/complete", post(complete_task))
}
