use sqlx::FromRow;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, FromRow)]
pub struct AuthToken {
    pub id: i64, 
    pub user_id: String,
    pub token: String,
    pub expires_at: i64, 
}

