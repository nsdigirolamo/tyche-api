use chrono::{Duration, Utc};
use jsonwebtoken::EncodingKey;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::errors::AuthError;

const JWT_ALGORITHM: jsonwebtoken::Algorithm = jsonwebtoken::Algorithm::HS512;

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    iss: String,      // Issuer
    sub: String,      // Subject
    aud: Vec<String>, // Audience (Recipients of the JWT)
    exp: i64,         // Expiration Time (JWT must not be accepted after this time)
    nbf: i64,         // Not Before (JWT must not be accepted before this time)
    iat: i64,         // Issued At
    jti: String,      // JWT ID (Unique identifier for the JWT)
}

pub fn encode_claims(user_id: Uuid) -> Result<String, AuthError> {
    let issued_at = Utc::now();
    let not_before = issued_at;
    let expiration = match issued_at.checked_add_signed(Duration::seconds(60)) {
        Some(time) => time,
        None => return Err(AuthError::Unspecified("an auth error occurred".to_string())),
    };

    let claims = Claims {
        iss: "api.tyche.social".to_string(),
        sub: user_id.to_string(),
        aud: vec!["api.tyche.social".to_string()],
        exp: expiration.timestamp(),
        nbf: not_before.timestamp(),
        iat: issued_at.timestamp(),
        jti: Uuid::new_v4().to_string(),
    };

    let header = jsonwebtoken::Header::new(JWT_ALGORITHM);
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET to be defined");
    let key = EncodingKey::from_secret(secret.as_ref());

    match jsonwebtoken::encode(&header, &claims, &key) {
        Ok(token) => Ok(token),
        Err(_) => Err(AuthError::Unspecified("an auth error occurred".to_string())),
    }
}

pub fn decode_claims(token: String) -> Result<Claims, AuthError> {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET to be defined");
    let key = jsonwebtoken::DecodingKey::from_secret(secret.as_ref());
    let validation = jsonwebtoken::Validation::new(JWT_ALGORITHM);

    match jsonwebtoken::decode::<Claims>(&token, &key, &validation) {
        Ok(data) => Ok(data.claims),
        Err(_) => Err(AuthError::InvalidToken(
            "the provided token could not be decoded".to_string(),
        )),
    }
}
