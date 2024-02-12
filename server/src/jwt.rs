use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use actix_web::cookie::{Cookie, SameSite};
use jsonwebtoken::{
    decode, encode, errors::Error as JwtError, Algorithm, DecodingKey, EncodingKey, Header,
    Validation, errors::ErrorKind,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn verify_jwt_token(token: &str) -> Result<Claims, JwtError> {
    let secret_key = secret_key().expect("Secret key must be set");
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret_key.as_ref()),
        &Validation::new(Algorithm::HS256),
    );

    match token_data {
        Ok(data) => Ok(data.claims),
        Err(err) => match *err.kind() {
            ErrorKind::ExpiredSignature => {
                println!("Token expired");
                Err(JwtError::from(ErrorKind::ExpiredSignature))
            }
            _ => {
                Err(err)
            }
        },
    }
}

pub fn generate_jwt_cookie(user_id: String) -> Cookie<'static> {
    let token = generate_jwt_token(user_id).expect("Failed to generate JWT");
    Cookie::build("auth_token", token)
        .path("/")
        .http_only(true)
        .same_site(SameSite::Strict)
        .finish()
}

fn generate_jwt_token(user_id: String) -> Result<String, JwtError> {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
        + jwt_expiration();

    let claims = Claims {
        sub: user_id,
        exp: expiration as usize,
    };

    let secret_key = secret_key().expect("Secret key must be set");

    encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(secret_key.as_ref()),
    )
}

fn secret_key() -> Result<Vec<u8>, env::VarError> {
    env::var("SECRET_KEY").map(|key| key.into_bytes())
}

fn jwt_expiration() -> u64 {
    env::var("JWT_EXPIRATION_SECONDS")
        .unwrap_or_else(|_| "3600".to_string())
        .parse()
        .expect("JWT_EXPIRATION must be a number")
}
