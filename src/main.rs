use crate::auth::router::auth_router;
use axum::Router;
use dotenv::dotenv;
use std::env;

mod auth;
mod models;
mod state;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABBASE_URL must be set.");
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set.");

    let db_pool = sqlx::PgPool::connect(&db_url)
        .await
        .expect("falied to connect to db.");

    let app_state = state::AppState::new(db_pool, jwt_secret);
    let app = Router::new()
        .nest("/auth", auth_router())
        .with_state(app_state);

    let addr: String = "127.0.0.1:3000".to_string();
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    println!("Listening on http://{}", &addr);

    axum::serve(listener, app).await.unwrap();
}
