use crate::state::AppState;
use crate::views::auth::{index_page, login_page, register_page};
use axum::{Router, routing::get};

pub fn views_router() -> Router<AppState> {
    Router::new()
        .route("/", get(index_page))
        .route("/auth/login", get(login_page))
        .route("/auth/register", get(register_page))
}
