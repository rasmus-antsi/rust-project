use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum::extract::Json;
use axum::response::IntoResponse;
use axum::{extract::State, http::StatusCode};
use serde_json::json;

use crate::auth::jwt::create_token;
use crate::models::user::{CreateUser, LoginUser, User};
use crate::state::AppState;

pub async fn register(
    State(state): State<AppState>,
    Json(body): Json<CreateUser>,
) -> impl IntoResponse {
    // Check if a user with this email already exists
    let existing = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(&body.email)
        .fetch_optional(&state.db_pool)
        .await;

    match existing {
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Database error"})),
            )
                .into_response();
        }
        Ok(Some(_)) => {
            return (
                StatusCode::CONFLICT,
                Json(json!({"error": "Email already in use"})),
            )
                .into_response();
        }
        Ok(None) => {} // good, continue
    }

    // Generate a random salt and hash the password using Argon2
    // Argon2 is a memory-hard hashing algorithm — much safer than bcrypt for passwords
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default().hash_password(body.password.as_bytes(), &salt);

    let password_hash = match password_hash {
        Ok(hash) => hash.to_string(),
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to hash password"})),
            )
                .into_response();
        }
    };

    // Insert the new user and return the created row using RETURNING *
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (username, email, password_hash) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(&body.username)
    .bind(&body.email)
    .bind(&password_hash)
    .fetch_one(&state.db_pool)
    .await;

    match user {
        Ok(user) => (StatusCode::CREATED, Json(json!({"user": user}))).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to create user"})),
        )
            .into_response(),
    }
}

pub async fn login(
    State(state): State<AppState>,
    Json(body): Json<LoginUser>,
) -> impl IntoResponse {
    // Try to find the user by email
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(&body.email)
        .fetch_optional(&state.db_pool)
        .await;

    let user = match user {
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Database error"})),
            )
                .into_response();
        }
        // If no user found, return 401 — don't say "email not found" to avoid leaking info
        Ok(None) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "Invalid credentials"})),
            )
                .into_response();
        }
        Ok(Some(u)) => u,
    };

    // Parse the stored hash and verify the provided password against it
    let parsed_hash = PasswordHash::new(&user.password_hash);
    let parsed_hash = match parsed_hash {
        Ok(h) => h,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to parse password hash"})),
            )
                .into_response();
        }
    };

    let valid = Argon2::default()
        .verify_password(body.password.as_bytes(), &parsed_hash)
        .is_ok();

    if !valid {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({"error": "Invalid credentials"})),
        )
            .into_response();
    }

    // Password is correct — create and return a JWT
    match create_token(user.id, &state.jwt_secret) {
        Ok(token) => (StatusCode::OK, Json(json!({"token": token}))).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to create token"})),
        )
            .into_response(),
    }
}
