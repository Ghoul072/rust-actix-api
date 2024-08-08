use std::env;
use jsonwebtoken::{ decode, encode, errors::Result, DecodingKey, EncodingKey, Header, Validation };
use serde::{ Deserialize, Serialize };

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub uuid: String,
    pub exp: usize,
}
pub fn create_token(uuid: &str) -> Result<String> {
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let expiration = chrono::Utc
        ::now()
        .checked_add_signed(chrono::Duration::seconds(3600))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        uuid: uuid.to_owned(),
        exp: expiration as usize,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(jwt_secret.as_ref()))
}

pub fn verify_auth_token(token: &str) -> Result<String> {
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let token = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(jwt_secret.as_ref()),
        &Validation::default()
    )?;

    return Ok(token.claims.uuid);
}
