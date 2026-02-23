use axum::{Json, extract::FromRequestParts, http::StatusCode, http::request::Parts};
use serde_json::json;
use uuid::Uuid;

use crate::auth::jwt::decode_token;
use crate::state::AppState;

pub struct AuthUser {
    pub user_id: Uuid,
}

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = (StatusCode, Json<serde_json::Value>);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        // Same as request.META.get('HTTP_AUTHORIZATION') in Django
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok()); // convert bytes to &str

        // Make sure it exists and starts with "Bearer "
        let token = match auth_header {
            Some(h) if h.starts_with("Bearer ") => &h[7..], // slice off "Bearer "
            _ => {
                return Err((
                    StatusCode::UNAUTHORIZED,
                    Json(json!({"error": "Missing or invalid Authorization header"})),
                ));
            }
        };

        // Decode and validate the JWT — like Django's TokenAuthentication
        let claims = decode_token(token, &state.jwt_secret).map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "Invalid or expired token"})),
            )
        })?;

        // Parse the UUID from the "sub" field in the token claims
        let user_id = Uuid::parse_str(&claims.sub).map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "Invalid user ID in token"})),
            )
        })?;

        Ok(AuthUser { user_id })
    }
}
