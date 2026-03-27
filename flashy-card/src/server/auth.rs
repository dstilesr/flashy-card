use axum::{
    extract::{Request, State},
    response::{IntoResponse, Redirect, Response},
    middleware::Next,
};
use axum_extra::extract::CookieJar;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use sqlx::postgres::PgPool;
use std::time::{SystemTime, UNIX_EPOCH};

use super::super::types::{AuthenticatedUser, Claims};
use super::super::repository;

pub const COOKIE_NAME: &str = "flashy_token";
const TOKEN_EXPIRY_SECS: u64 = 60 * 60 * 24; // 24 hours

/// Wrapper type for the JWT secret string, stored in Axum Extension layer
#[derive(Clone)]
pub struct JwtSecret(pub String);

/// Read the JWT secret from the environment. Panics at startup if not set.
pub fn get_jwt_secret() -> String {
    std::env::var("JWT_SECRET")
        .expect("JWT_SECRET environment variable must be set")
}

/// Create a signed JWT for the given user
pub fn create_token(username: &str, uuid: &str, secret: &str) -> Result<String, String> {
    let exp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| format!("System time error: {}", e))?
        .as_secs() + TOKEN_EXPIRY_SECS;

    let claims = Claims {
        sub: username.to_string(),
        uuid: uuid.to_string(),
        exp: exp as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| format!("Failed to create token: {}", e))
}

/// Check if the request has a valid JWT cookie. Used for conditional auth in public routes.
pub fn is_authenticated(jar: &CookieJar, secret: &str) -> bool {
    let token = match jar.get(COOKIE_NAME) {
        Some(cookie) => cookie.value().to_string(),
        None => return false,
    };
    decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .is_ok()
}

/// Determine where to send an unauthenticated user: /create-user if no users exist, else /login.
async fn unauthenticated_redirect(pool: &PgPool) -> Redirect {
    match repository::has_users(pool).await {
        Ok(false) => Redirect::to("/create-user"),
        _ => Redirect::to("/login"),
    }
}

/// Middleware: validates the JWT cookie on every request to a protected route.
/// Inserts AuthenticatedUser into request extensions on success.
/// Redirects to /create-user if no users exist, or /login otherwise.
pub async fn require_auth(
    State(pool): State<PgPool>,
    jar: CookieJar,
    mut request: Request,
    next: Next,
) -> Response {
    let secret = match request.extensions().get::<JwtSecret>() {
        Some(s) => s.0.clone(),
        None => {
            log::error!("JwtSecret extension not found in request");
            return unauthenticated_redirect(&pool).await.into_response();
        }
    };

    let token = match jar.get(COOKIE_NAME) {
        Some(cookie) => cookie.value().to_string(),
        None => {
            log::debug!("No auth cookie found");
            return unauthenticated_redirect(&pool).await.into_response();
        }
    };

    match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    ) {
        Ok(token_data) => {
            let user = AuthenticatedUser {
                username: token_data.claims.sub,
                uuid: token_data.claims.uuid,
            };
            request.extensions_mut().insert(user);
            next.run(request).await
        }
        Err(e) => {
            log::debug!("Invalid JWT: {}", e);
            unauthenticated_redirect(&pool).await.into_response()
        }
    }
}