use crate::models::authtoken::AuthToken;
use crate::models::user::User;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::env;
use uuid::Uuid;

pub async fn find_user_by_id(pool: &SqlitePool, id: &str) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = ?", id)
        .fetch_one(pool)
        .await?;

    Ok(user)
}

pub async fn find_authtoken_by_user_id(
    pool: &SqlitePool,
    user_id: &str,
) -> Result<AuthToken, sqlx::Error> {
    let authtoken = sqlx::query!(
        "SELECT id, user_id, token, expires_at FROM authtokens WHERE user_id = ?",
        user_id
    )
    .fetch_one(pool)
    .await?;

    Ok(AuthToken {
        id: authtoken.id,
        user_id: authtoken.user_id,
        token: authtoken.token,
        expires_at: authtoken.expires_at,
    })
}

pub async fn find_user_by_username(pool: &SqlitePool, username: &str) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE username = ?", username)
        .fetch_one(pool)
        .await?;

    Ok(user)
}

async fn create_authtoken_record(
    pool: &SqlitePool,
    user_id: &str,
) -> Result<AuthToken, sqlx::Error> {
    let token = "hey";
    let expires_at = 0;
    let id = sqlx::query!(
        "INSERT INTO authtokens (user_id, token, expires_at) VALUES (?, ?, ?)",
        user_id,
        token,
        expires_at
    )
    .execute(pool)
    .await?
    .last_insert_rowid();

    Ok(AuthToken {
        id,
        user_id: user_id.to_owned(),
        token: token.to_owned(),
        expires_at,
    })
}

pub async fn create_user_record(
    pool: &SqlitePool,
    username: &str,
    password_hash: &str,
) -> Result<User, sqlx::Error> {
    let id = generate_uuid();
    sqlx::query!(
        "INSERT INTO users (id, username, password_hash) VALUES (?, ?, ?)",
        id,
        username,
        password_hash
    )
    .execute(pool)
    .await?;

    create_authtoken_record(pool, &id).await?;

    Ok(User {
        id: id.to_owned(),
        username: username.to_owned(),
        password_hash: password_hash.to_owned(),
    })
}

pub async fn setup_database() -> SqlitePool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    SqlitePoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Failed to create pool")
}

fn generate_uuid() -> String {
    Uuid::new_v4().to_string()
}
