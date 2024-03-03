use crate::db::{create_user_record, find_user_by_username, find_authtoken_by_user_id};
use crate::jwt::{generate_jwt_cookie, verify_jwt_token, Claims};
use actix_web::{get, post, web, HttpRequest, HttpResponse};
use bcrypt::{hash, verify, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::env;

#[derive(Deserialize)]
pub struct AuthData {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct AuthenticatedResponse {
    claims: Claims,
    auth_token: String,
}

#[post("/register")]
pub async fn register_user(pool: web::Data<SqlitePool>, form: web::Json<AuthData>) -> HttpResponse {
    let password_hash = hash_password(form.password.as_str());
    match create_user_record(&pool, form.username.as_str(), password_hash.as_str()).await {
        Ok(user) => {
            let cookie = generate_jwt_cookie(user.id.to_string());
            HttpResponse::Ok().cookie(cookie).finish()
        }
        Err(e) => {
            println!("Failed to create user: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/login")]
pub async fn login_user(
    db_pool: web::Data<SqlitePool>,
    user_credentials: web::Json<AuthData>,
) -> HttpResponse {
    let user = match find_user_by_username(&db_pool, &user_credentials.username).await {
        Ok(user) => user,
        Err(_) => return HttpResponse::Unauthorized().finish(),
    };
    let is_password_correct =
        verify(&user_credentials.password, &user.password_hash).unwrap_or(false);
    if is_password_correct {
        let cookie = generate_jwt_cookie(user.id.to_string());
        HttpResponse::Ok().cookie(cookie).finish()
    } else {
        HttpResponse::Unauthorized().finish()
    }
}

#[get("/authenticated")]
pub async fn is_authenticated(req: HttpRequest, db_pool: web::Data<SqlitePool>) -> HttpResponse {
    if let Some(cookie) = req.cookie("auth_token") {
        let token = cookie.value();
        match verify_jwt_token(token) {
            Ok(claims) => {
                let auth_token = find_authtoken_by_user_id(&db_pool, &claims.sub).await;
                println!("Token is valid and not expired: {:?}", claims);
                HttpResponse::Ok().json(AuthenticatedResponse {
                    claims,
                    auth_token: auth_token.unwrap().token,
                })

            }
            Err(err) => {
                println!("Error verifying token: {:?}", err);
                HttpResponse::Unauthorized().body("Invalid token")
            }
        }
    } else {
        HttpResponse::Unauthorized().body("No token provided")
    }
}

fn hash_password(password: &str) -> String {
    hash(password, DEFAULT_COST).unwrap()
}

fn secret_key() -> Result<Vec<u8>, env::VarError> {
    let secret = env::var("SECRET_KEY")?;
    Ok(secret.into_bytes())
}
