use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use sqlx::postgres::PgPool;
use uuid::Uuid;

use super::super::types;


/// Check if any users exist in the database
pub async fn has_users(pool: &PgPool) -> Result<bool, String> {
    let result = sqlx::query!("SELECT EXISTS(SELECT 1 FROM users) AS has_users")
        .fetch_one(pool)
        .await
        .map_err(|e| {
            log::error!("Error checking for users: {}", e);
            format!("Could not check for existing users: {}", e)
        })?;
    Ok(result.has_users.unwrap_or(false))
}


/// Create a new user in the database
pub async fn create_user(username: String, password: String, pool: &PgPool) -> Result<(), String> {
    let existing = sqlx::query!("select count(id) as total from users where username = $1", username)
        .fetch_one(pool)
        .await
        .map_err(|e| {
            log::error!("Error counting users: {}", e);
            format!("Could not check for existing users: {}", e)
        })?
        .total
        .unwrap_or(0);

    if existing > 0 {
        return Err(format!("Username {} already exists", username));
    }

    let uuid = Uuid::new_v4().to_string();
    let salt = SaltString::generate(&mut OsRng);
    let a2 = Argon2::default();
    let pw_hash = a2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| {
            log::error!("Error hashing password: {}", e);
            format!("Could not hash password: {}", e)
        })?
        .to_string();

    sqlx::query!(
            "insert into users (username, uuid, pw_hash) VALUES ($1, $2, $3);",
            username,
            uuid,
            pw_hash
        )
        .execute(pool)
        .await
        .map_err(|e| {
            log::error!("Error creating user: {}", e);
            format!("Could not create user: {}", e)
        })?;

    log::info!("Created user {} ({})", username, uuid);
    Ok(())
}


/// Validate login for the user, returning the UserInfo on success
pub async fn validate_password(username: String, given_password: String, pool: &PgPool) -> Result<types::UserInfo, String> {

    let user = sqlx::query_as::<_, types::UserInfo>("select * from users where username = $1")
        .bind(username)
        .fetch_one(pool)
        .await
        .map_err(|e| {
            log::error!("Error retrieving user: {}", e);
            format!("Could not retrieve user from database: {}", e)
        })?;

    let hashed = PasswordHash::new(&user.pw_hash).map_err(|e| {
        log::error!("Error hashing password: {}", e);
        format!("Could not create password hash: {}", e)
    })?;
    Argon2::default().verify_password(given_password.as_bytes(), &hashed).map_err(|e| {
        log::error!("Error verifying password: {}", e);
        format!("Invalid password: {}", e)
    })?;

    Ok(user)
}
