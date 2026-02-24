use axum::{
    Router,
    routing::{delete, get, post},
};

use crate::pomodoro::handlers::{delete_session, end_session, get_sessions, start_session};
use crate::state::AppState;

pub fn pomodoro_router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_sessions))
        .route("/start", post(start_session))
        .route("/{id}/end", post(end_session))
        .route("/{id}", delete(delete_session))
}
