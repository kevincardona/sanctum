
use std::env;
use uuid::Uuid;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use crate::models::user::User;

pub async fn create_user(
    pool: &SqlitePool,
    email: &str,
    password_hash: &str,
) -> Result<User, sqlx::Error> {
    sqlx::query!(
        "INSERT INTO users (email, password_hash) VALUES (?, ?)",
        email,
        password_hash
    )
    .execute(pool)
    .await?;

    Ok(User {
        id: generate_uuid(),
        email: email.to_owned(),
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
