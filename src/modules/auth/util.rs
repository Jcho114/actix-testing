use chrono::Utc;
use jsonwebtoken::{
    decode, encode, errors::Error, Algorithm, DecodingKey, EncodingKey, Header, TokenData,
    Validation,
};
use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};

use crate::core::config::CONFIG;

#[derive(Serialize, Deserialize)]
pub struct AccessTokenClaims {
    token_type: String,
    sub: i32,
    iat: i64,
    exp: i64,
}

#[derive(Serialize, Deserialize)]
pub struct RefreshTokenClaims {
    token_type: String,
    pub sub: i32,
    iat: i64,
    exp: i64,
}

const ACCESS_EXPIRATION_MINUTES: i64 = 60;
const REFRESH_EXPIRATION_HOURS: i64 = 24;

pub fn generate_access_token(user_id: i32) -> Result<String, Error> {
    let iat = Utc::now().timestamp_millis();
    let exp = iat + (1000 * 60 * ACCESS_EXPIRATION_MINUTES);
    let token_type = String::from("access");

    let claims = AccessTokenClaims {
        token_type,
        iat,
        exp,
        sub: user_id,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(CONFIG.jwt_secret.expose_secret().as_bytes()),
    )
}

pub fn generate_refresh_token(user_id: i32) -> Result<String, Error> {
    let iat = Utc::now().timestamp_millis();
    let exp = iat + (1000 * 60 * 60 * REFRESH_EXPIRATION_HOURS);
    let token_type = String::from("refresh");

    let claims = RefreshTokenClaims {
        token_type,
        iat,
        exp,
        sub: user_id,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(CONFIG.jwt_secret.expose_secret().as_bytes()),
    )
}

pub fn validate_access_token(access_token: String) -> Option<TokenData<AccessTokenClaims>> {
    let result = decode::<AccessTokenClaims>(
        access_token.as_str(),
        &DecodingKey::from_secret(CONFIG.jwt_secret.expose_secret().as_bytes()),
        &Validation::new(Algorithm::HS256),
    );

    if let Err(_) = result {
        return None;
    }

    let token_data = result.unwrap();

    let now = Utc::now().timestamp_millis();
    if now > token_data.claims.exp {
        return None;
    }

    return Some(token_data);
}

pub fn validate_refresh_token(refresh_token: String) -> Option<TokenData<RefreshTokenClaims>> {
    let result = decode::<RefreshTokenClaims>(
        refresh_token.as_str(),
        &DecodingKey::from_secret(CONFIG.jwt_secret.expose_secret().as_bytes()),
        &Validation::new(Algorithm::HS256),
    );

    if let Err(_) = result {
        return None;
    }

    let token_data = result.unwrap();

    let now = Utc::now().timestamp_millis();
    if now > token_data.claims.exp {
        return None;
    }

    return Some(token_data);
}
