use axum::{Router, routing::post};

use crate::auth::handlers::{login, register};
use crate::state::AppState;

pub fn auth_router() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
}
