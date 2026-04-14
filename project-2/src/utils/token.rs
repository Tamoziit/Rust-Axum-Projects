/* JWT Token Utils */
use axum::http::StatusCode;
use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

use crate::errors::{ErrorMessage, HttpError};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims { // data to be encoded in JWT
    pub sub: String, // subject/User ID
    pub iat: usize, // issued at timestamp
    pub exp: usize, // expiry
}

pub fn create_token(
    user_id: &str,
    secret: &[u8],
    expires_in_seconds: i64,
) -> Result<String, jsonwebtoken::errors::Error> {
    if user_id.is_empty() {
        return Err(jsonwebtoken::errors::ErrorKind::InvalidSubject.into());
    }

    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::minutes(expires_in_seconds)).timestamp() as usize;
    let claims = TokenClaims {
        sub: user_id.to_string(),
        iat,
        exp,
    };

    encode( // encoding the token
        &Header::default(), // default header
        &claims, // JWT claims
        &EncodingKey::from_secret(secret), // JWT Secret
    ) // on success returns a String
}

pub fn decode_token<T: Into<String>>(
    token: T,
    secret: &[u8]
) -> Result<String, HttpError> {
    let decode = decode::<TokenClaims>(
        &token.into(),
        &DecodingKey::from_secret(secret),
        &Validation::new(Algorithm::HS256)
    ); // decoding token using HS256 algo

    match decode {
        Ok(token) => Ok(token.claims.sub),
        Err(_) => Err(HttpError::new(ErrorMessage::InvalidToken.to_string(), StatusCode::UNAUTHORIZED))
    }
}