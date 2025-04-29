use std::{env, fs};

use chrono::{Duration, Utc};
use jsonwebtoken::EncodingKey;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::errors::AuthError;

const JWT_ALGORITHM: jsonwebtoken::Algorithm = jsonwebtoken::Algorithm::HS512;
const ISSUER: &str = "api.tyche.social";

static JWT_SECRET: Lazy<String> = Lazy::new(|| {
    fs::read_to_string(env::var("JWT_SECRET_FILE").expect("JWT_SECRET_FILE must be set"))
        .expect("JWT_SECRET_FILE must be readable")
});

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub iss: String,      // Issuer
    pub sub: String,      // Subject
    pub aud: Vec<String>, // Audience (Recipients of the JWT)
    pub exp: i64,         // Expiration Time (JWT must not be accepted after this time)
    pub nbf: i64,         // Not Before (JWT must not be accepted before this time)
    pub iat: i64,         // Issued At
    pub jti: String,      // JWT ID (Unique identifier for the JWT)
}

pub fn encode_claims(user_id: Uuid) -> Result<String, AuthError> {
    let issued_at = Utc::now();
    let not_before = issued_at;
    let expiration = match issued_at.checked_add_signed(Duration::days(7)) {
        Some(time) => time,
        None => return Err(AuthError::Unspecified("an auth error occurred".to_string())),
    };

    let claims = Claims {
        iss: ISSUER.to_string(),
        sub: user_id.to_string(),
        aud: vec![ISSUER.to_string()],
        exp: expiration.timestamp(),
        nbf: not_before.timestamp(),
        iat: issued_at.timestamp(),
        jti: Uuid::new_v4().to_string(),
    };

    let header = jsonwebtoken::Header::new(JWT_ALGORITHM);
    let key = EncodingKey::from_secret(JWT_SECRET.as_bytes());

    match jsonwebtoken::encode(&header, &claims, &key) {
        Ok(token) => Ok(token),
        Err(_) => Err(AuthError::Unspecified("an auth error occurred".to_string())),
    }
}

pub fn decode_claims(token: String) -> Result<Claims, AuthError> {
    let key = jsonwebtoken::DecodingKey::from_secret(JWT_SECRET.as_bytes());
    let mut validation = jsonwebtoken::Validation::new(JWT_ALGORITHM);
    validation.set_issuer(&[ISSUER]);
    validation.set_audience(&[ISSUER]);

    match jsonwebtoken::decode::<Claims>(&token, &key, &validation) {
        Ok(data) => Ok(data.claims),
        Err(err) => {
            println!("{}", err);

            Err(AuthError::InvalidToken(
                "the provided token could not be decoded".to_string(),
            ))
        }
    }
}
