use std::env;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use jsonwebtoken::{ decode, encode, errors::Result, DecodingKey, EncodingKey, Header, Validation };
use r2d2::PooledConnection;
use serde::{ Deserialize, Serialize };
use ::uuid::Uuid;

use crate::utils::auth::get_user_id_from_uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_uuid: Uuid,
    pub token_type: String,
    pub exp: i64,
}

pub fn create_token(user_uuid: Uuid, token_type: &str) -> Result<String> {
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let expiration = match token_type {
        "access" =>
            chrono::Utc
                ::now()
                .checked_add_signed(chrono::Duration::hours(1))
                .expect("valid timestamp")
                .timestamp(),
        "refresh" =>
            chrono::Utc
                ::now()
                .checked_add_signed(chrono::Duration::days(7))
                .expect("valid timestamp")
                .timestamp(),
        _ => panic!("Invalid token type"),
    };

    let claims = Claims {
        user_uuid: user_uuid.to_owned(),
        token_type: token_type.to_owned(),
        exp: expiration,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(jwt_secret.as_ref()))
}

pub fn verify_auth_token(
    token: &str,
    connection: PooledConnection<ConnectionManager<PgConnection>>
) -> Result<i32> {
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let token = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(jwt_secret.as_ref()),
        &Validation::default()
    )?;

    return Ok(get_user_id_from_uuid(token.claims.user_uuid, connection));
}
