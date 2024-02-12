use sqlx::FromRow;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password_hash: String,
}
