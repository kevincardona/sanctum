use crate::db::create_user;
use actix_web::{web, HttpResponse, cookie::Cookie};
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, EncodingKey, Header, decode, DecodingKey, Validation, Algorithm};
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::env;

#[derive(Deserialize)]
pub struct AuthData {
    email: String,
    password: String,
}

#[derive(Deserialize, Serialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub async fn register_user(pool: web::Data<SqlitePool>, form: web::Json<AuthData>) -> HttpResponse {
    let password_hash = hash_password(form.password.as_str());
    create_user(&pool, form.email.as_str(), password_hash.as_str())
        .await
        .unwrap();
    let cookie = generate_jwt_cookie(form.email.to_string());
    HttpResponse::Ok().cookie(cookie).finish()
}

pub async fn login_user(
    db_pool: web::Data<SqlitePool>,
    user_credentials: web::Json<AuthData>,
) -> HttpResponse {
    let user = match sqlx::query!(
        "SELECT id, email, password_hash FROM users WHERE email = ?",
        user_credentials.email
    )
    .fetch_one(db_pool.get_ref())
    .await
    {
        Ok(record) => record,
        Err(_) => return HttpResponse::Unauthorized().finish(),
    };

    let is_password_correct =
        verify(&user_credentials.password, &user.password_hash).unwrap_or(false);

    if is_password_correct {
        let cookie = generate_jwt_cookie(user.email);
        HttpResponse::Ok().cookie(cookie).finish()
    } else {
        HttpResponse::Unauthorized().finish()
    }
}

fn verify_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let validation = Validation::new(Algorithm::HS256);
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(env::var("SECRET_KEY").unwrap().as_ref()),
        &validation,
    )?;
    
    Ok(token_data.claims)
}

fn generate_jwt_cookie(email: String) -> Cookie<'static> {
    let token = generate_jwt(email);
    Cookie::build("auth_token", token)
        .path("/")
        .http_only(true)
        .finish()
}

fn generate_jwt(email: String) -> String {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() + 3600;

    encode(
        &Header::default(),
        &Claims {
            sub: email,
            exp: expiration as usize, 
        },
        &EncodingKey::from_secret(env::var("SECRET_KEY").unwrap().as_ref()),
    )
    .unwrap()
}

fn hash_password(password: &str) -> String {
    hash(password, DEFAULT_COST).unwrap()
}
